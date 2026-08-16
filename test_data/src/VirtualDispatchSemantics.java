import dispatch.base.PackageBase;
import dispatch.base.ProtectedBridge;
import dispatch.base.PublicBridge;
import dispatch.base.SamePackageChild;
import dispatch.foreign.ForeignChild;
import dispatch.foreign.ForeignProtectedGrandchild;
import dispatch.foreign.ForeignPublicGrandchild;
import dispatch.interfaces.InterfaceImplementation;
import dispatch.interfaces.InterfaceOwner;

public class VirtualDispatchSemantics {
    public static void main(String[] args) {
        PackageBase foreign = new ForeignChild();
        PackageBase samePackage = new SamePackageChild();
        PackageBase publicIntermediate = new ForeignPublicGrandchild();
        PackageBase protectedIntermediate = new ForeignProtectedGrandchild();
        InterfaceOwner inheritedInterfaceMethod = new InterfaceImplementation();

        System.out.println(foreign.call());
        System.out.println(samePackage.call());
        System.out.println(publicIntermediate.call());
        System.out.println(protectedIntermediate.call());
        System.out.println(inheritedInterfaceMethod.operation());
    }
}
