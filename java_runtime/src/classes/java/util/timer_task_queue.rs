use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::FieldAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::util::TimerTask};

// class java.util.Timer$TaskQueue
pub struct TimerTaskQueue;

impl TimerTaskQueue {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Timer$TaskQueue",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![
                JavaFieldProto::new("queue", "[Ljava/util/TimerTask;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("size", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Timer$TaskQueue::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let queue = jvm.instantiate_array("Ljava/util/TimerTask;", 128).await?;
        jvm.put_field(&mut this, "queue", "[Ljava/util/TimerTask;", queue).await?;
        Ok(())
    }

    pub(crate) async fn add(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, task: ClassInstanceRef<TimerTask>) -> Result<bool> {
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        let mut queue: ClassInstanceRef<Array<TimerTask>> = jvm.get_field(this, "queue", "[Ljava/util/TimerTask;").await?;
        if size as usize + 1 == jvm.array_length(&queue).await? {
            let old_length = jvm.array_length(&queue).await?;
            let mut expanded: ClassInstanceRef<Array<TimerTask>> = jvm.instantiate_array("Ljava/util/TimerTask;", old_length * 2).await?.into();
            let tasks: Vec<ClassInstanceRef<TimerTask>> = jvm.load_array(&queue, 0, size as usize + 1).await?;
            jvm.store_array(&mut expanded, 0, tasks).await?;
            queue = expanded;
            jvm.put_field(this, "queue", "[Ljava/util/TimerTask;", queue.clone()).await?;
        }

        let mut child = size as usize + 1;
        while child > 1 {
            let parent = child / 2;
            let parent_task: ClassInstanceRef<TimerTask> = jvm.load_array(&queue, parent, 1).await?.remove(0);
            let parent_time: i64 = jvm.get_field(&parent_task, "nextExecutionTime", "J").await?;
            let task_time: i64 = jvm.get_field(&task, "nextExecutionTime", "J").await?;
            if parent_time <= task_time {
                break;
            }
            jvm.store_array(&mut queue, child, core::iter::once(parent_task)).await?;
            child = parent;
        }
        jvm.store_array(&mut queue, child, core::iter::once(task)).await?;
        jvm.put_field(this, "size", "I", size + 1).await?;
        Ok(child == 1)
    }

    pub(crate) async fn remove_min(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        if size == 0 {
            return Ok(());
        }

        let mut queue: ClassInstanceRef<Array<TimerTask>> = jvm.get_field(this, "queue", "[Ljava/util/TimerTask;").await?;
        let last: ClassInstanceRef<TimerTask> = jvm.load_array(&queue, size as usize, 1).await?.remove(0);
        let null: ClassInstanceRef<TimerTask> = None.into();
        jvm.store_array(&mut queue, size as usize, core::iter::once(null)).await?;
        let new_size = size - 1;
        jvm.put_field(this, "size", "I", new_size).await?;
        if new_size == 0 {
            return Ok(());
        }

        let last_time: i64 = jvm.get_field(&last, "nextExecutionTime", "J").await?;
        let mut parent = 1usize;
        while parent * 2 <= new_size as usize {
            let mut child = parent * 2;
            let mut child_task: ClassInstanceRef<TimerTask> = jvm.load_array(&queue, child, 1).await?.remove(0);
            let mut child_time: i64 = jvm.get_field(&child_task, "nextExecutionTime", "J").await?;
            if child < new_size as usize {
                let right: ClassInstanceRef<TimerTask> = jvm.load_array(&queue, child + 1, 1).await?.remove(0);
                let right_time: i64 = jvm.get_field(&right, "nextExecutionTime", "J").await?;
                if right_time < child_time {
                    child += 1;
                    child_task = right;
                    child_time = right_time;
                }
            }
            if last_time <= child_time {
                break;
            }
            jvm.store_array(&mut queue, parent, core::iter::once(child_task)).await?;
            parent = child;
        }
        jvm.store_array(&mut queue, parent, core::iter::once(last)).await
    }

    pub(crate) async fn reschedule_min(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, new_time: i64) -> Result<()> {
        let mut queue: ClassInstanceRef<Array<TimerTask>> = jvm.get_field(this, "queue", "[Ljava/util/TimerTask;").await?;
        let mut task: ClassInstanceRef<TimerTask> = jvm.load_array(&queue, 1, 1).await?.remove(0);
        jvm.put_field(&mut task, "nextExecutionTime", "J", new_time).await?;

        let size: i32 = jvm.get_field(this, "size", "I").await?;
        let mut parent = 1usize;
        while parent * 2 <= size as usize {
            let mut child = parent * 2;
            let mut child_task: ClassInstanceRef<TimerTask> = jvm.load_array(&queue, child, 1).await?.remove(0);
            let mut child_time: i64 = jvm.get_field(&child_task, "nextExecutionTime", "J").await?;
            if child < size as usize {
                let right: ClassInstanceRef<TimerTask> = jvm.load_array(&queue, child + 1, 1).await?.remove(0);
                let right_time: i64 = jvm.get_field(&right, "nextExecutionTime", "J").await?;
                if right_time < child_time {
                    child += 1;
                    child_task = right;
                    child_time = right_time;
                }
            }
            if new_time <= child_time {
                break;
            }
            jvm.store_array(&mut queue, parent, core::iter::once(child_task)).await?;
            parent = child;
        }
        jvm.store_array(&mut queue, parent, core::iter::once(task)).await
    }

    pub(crate) async fn clear(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        if size > 0 {
            let mut queue: ClassInstanceRef<Array<TimerTask>> = jvm.get_field(this, "queue", "[Ljava/util/TimerTask;").await?;
            let nulls = core::iter::repeat_with(|| ClassInstanceRef::<TimerTask>::from(None)).take(size as usize);
            jvm.store_array(&mut queue, 1, nulls).await?;
            jvm.put_field(this, "size", "I", 0i32).await?;
        }
        Ok(())
    }
}
