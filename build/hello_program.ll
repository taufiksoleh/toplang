; ModuleID = 'TopLang Module'
source_filename = "TopLang Module"

@str = private unnamed_addr constant [14 x i8] c"Hello, World!\00", align 1
@str.1 = private unnamed_addr constant [9 x i8] c"Count is\00", align 1
@str.2 = private unnamed_addr constant [17 x i8] c"Count reached 5!\00", align 1
@str.3 = private unnamed_addr constant [22 x i8] c"Something went wrong!\00", align 1

declare void @printDouble(double)

declare void @printString(ptr)

define double @main() {
entry:
  %count = alloca double, align 8
  call void @printString(ptr @str)
  store double 1.000000e+00, ptr %count, align 8
  br label %loopcond

loopcond:                                         ; preds = %loop, %entry
  %count1 = load double, ptr %count, align 8
  %lttmp = fcmp olt double %count1, 5.000000e+00
  br i1 %lttmp, label %loop, label %afterloop

loop:                                             ; preds = %loopcond
  call void @printString(ptr @str.1)
  %count2 = load double, ptr %count, align 8
  call void @printDouble(double %count2)
  %count3 = load double, ptr %count, align 8
  %count4 = load double, ptr %count, align 8
  %addtmp = fadd double %count4, 1.000000e+00
  store double %addtmp, ptr %count, align 8
  br label %loopcond

afterloop:                                        ; preds = %loopcond
  %count5 = load double, ptr %count, align 8
  %eqtmp = fcmp oeq double %count5, 5.000000e+00
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %afterloop
  call void @printString(ptr @str.2)
  br label %ifcont

else:                                             ; preds = %afterloop
  call void @printString(ptr @str.3)
  br label %ifcont

ifcont:                                           ; preds = %else, %then
  ret double 0.000000e+00
}
