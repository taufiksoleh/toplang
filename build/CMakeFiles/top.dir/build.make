# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.31

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/local/bin/cmake

# The command to remove a file.
RM = /usr/local/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/taufiksoleh/Personal/Compiler/toplang

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/taufiksoleh/Personal/Compiler/toplang/build

# Include any dependencies generated for this target.
include CMakeFiles/top.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/top.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/top.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/top.dir/flags.make

CMakeFiles/top.dir/codegen:
.PHONY : CMakeFiles/top.dir/codegen

CMakeFiles/top.dir/src/main.cpp.o: CMakeFiles/top.dir/flags.make
CMakeFiles/top.dir/src/main.cpp.o: /Users/taufiksoleh/Personal/Compiler/toplang/src/main.cpp
CMakeFiles/top.dir/src/main.cpp.o: CMakeFiles/top.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/taufiksoleh/Personal/Compiler/toplang/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/top.dir/src/main.cpp.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/top.dir/src/main.cpp.o -MF CMakeFiles/top.dir/src/main.cpp.o.d -o CMakeFiles/top.dir/src/main.cpp.o -c /Users/taufiksoleh/Personal/Compiler/toplang/src/main.cpp

CMakeFiles/top.dir/src/main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/top.dir/src/main.cpp.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/taufiksoleh/Personal/Compiler/toplang/src/main.cpp > CMakeFiles/top.dir/src/main.cpp.i

CMakeFiles/top.dir/src/main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/top.dir/src/main.cpp.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/taufiksoleh/Personal/Compiler/toplang/src/main.cpp -o CMakeFiles/top.dir/src/main.cpp.s

CMakeFiles/top.dir/src/lexer.cpp.o: CMakeFiles/top.dir/flags.make
CMakeFiles/top.dir/src/lexer.cpp.o: /Users/taufiksoleh/Personal/Compiler/toplang/src/lexer.cpp
CMakeFiles/top.dir/src/lexer.cpp.o: CMakeFiles/top.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/taufiksoleh/Personal/Compiler/toplang/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/top.dir/src/lexer.cpp.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/top.dir/src/lexer.cpp.o -MF CMakeFiles/top.dir/src/lexer.cpp.o.d -o CMakeFiles/top.dir/src/lexer.cpp.o -c /Users/taufiksoleh/Personal/Compiler/toplang/src/lexer.cpp

CMakeFiles/top.dir/src/lexer.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/top.dir/src/lexer.cpp.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/taufiksoleh/Personal/Compiler/toplang/src/lexer.cpp > CMakeFiles/top.dir/src/lexer.cpp.i

CMakeFiles/top.dir/src/lexer.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/top.dir/src/lexer.cpp.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/taufiksoleh/Personal/Compiler/toplang/src/lexer.cpp -o CMakeFiles/top.dir/src/lexer.cpp.s

CMakeFiles/top.dir/src/parser.cpp.o: CMakeFiles/top.dir/flags.make
CMakeFiles/top.dir/src/parser.cpp.o: /Users/taufiksoleh/Personal/Compiler/toplang/src/parser.cpp
CMakeFiles/top.dir/src/parser.cpp.o: CMakeFiles/top.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/taufiksoleh/Personal/Compiler/toplang/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/top.dir/src/parser.cpp.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/top.dir/src/parser.cpp.o -MF CMakeFiles/top.dir/src/parser.cpp.o.d -o CMakeFiles/top.dir/src/parser.cpp.o -c /Users/taufiksoleh/Personal/Compiler/toplang/src/parser.cpp

CMakeFiles/top.dir/src/parser.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/top.dir/src/parser.cpp.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/taufiksoleh/Personal/Compiler/toplang/src/parser.cpp > CMakeFiles/top.dir/src/parser.cpp.i

CMakeFiles/top.dir/src/parser.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/top.dir/src/parser.cpp.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/taufiksoleh/Personal/Compiler/toplang/src/parser.cpp -o CMakeFiles/top.dir/src/parser.cpp.s

CMakeFiles/top.dir/src/ast.cpp.o: CMakeFiles/top.dir/flags.make
CMakeFiles/top.dir/src/ast.cpp.o: /Users/taufiksoleh/Personal/Compiler/toplang/src/ast.cpp
CMakeFiles/top.dir/src/ast.cpp.o: CMakeFiles/top.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/taufiksoleh/Personal/Compiler/toplang/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building CXX object CMakeFiles/top.dir/src/ast.cpp.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/top.dir/src/ast.cpp.o -MF CMakeFiles/top.dir/src/ast.cpp.o.d -o CMakeFiles/top.dir/src/ast.cpp.o -c /Users/taufiksoleh/Personal/Compiler/toplang/src/ast.cpp

CMakeFiles/top.dir/src/ast.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/top.dir/src/ast.cpp.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/taufiksoleh/Personal/Compiler/toplang/src/ast.cpp > CMakeFiles/top.dir/src/ast.cpp.i

CMakeFiles/top.dir/src/ast.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/top.dir/src/ast.cpp.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/taufiksoleh/Personal/Compiler/toplang/src/ast.cpp -o CMakeFiles/top.dir/src/ast.cpp.s

CMakeFiles/top.dir/src/codegen.cpp.o: CMakeFiles/top.dir/flags.make
CMakeFiles/top.dir/src/codegen.cpp.o: /Users/taufiksoleh/Personal/Compiler/toplang/src/codegen.cpp
CMakeFiles/top.dir/src/codegen.cpp.o: CMakeFiles/top.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/taufiksoleh/Personal/Compiler/toplang/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building CXX object CMakeFiles/top.dir/src/codegen.cpp.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/top.dir/src/codegen.cpp.o -MF CMakeFiles/top.dir/src/codegen.cpp.o.d -o CMakeFiles/top.dir/src/codegen.cpp.o -c /Users/taufiksoleh/Personal/Compiler/toplang/src/codegen.cpp

CMakeFiles/top.dir/src/codegen.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/top.dir/src/codegen.cpp.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/taufiksoleh/Personal/Compiler/toplang/src/codegen.cpp > CMakeFiles/top.dir/src/codegen.cpp.i

CMakeFiles/top.dir/src/codegen.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/top.dir/src/codegen.cpp.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/taufiksoleh/Personal/Compiler/toplang/src/codegen.cpp -o CMakeFiles/top.dir/src/codegen.cpp.s

# Object files for target top
top_OBJECTS = \
"CMakeFiles/top.dir/src/main.cpp.o" \
"CMakeFiles/top.dir/src/lexer.cpp.o" \
"CMakeFiles/top.dir/src/parser.cpp.o" \
"CMakeFiles/top.dir/src/ast.cpp.o" \
"CMakeFiles/top.dir/src/codegen.cpp.o"

# External object files for target top
top_EXTERNAL_OBJECTS =

top: CMakeFiles/top.dir/src/main.cpp.o
top: CMakeFiles/top.dir/src/lexer.cpp.o
top: CMakeFiles/top.dir/src/parser.cpp.o
top: CMakeFiles/top.dir/src/ast.cpp.o
top: CMakeFiles/top.dir/src/codegen.cpp.o
top: CMakeFiles/top.dir/build.make
top: /usr/local/opt/llvm/lib/libLLVMCore.a
top: /usr/local/opt/llvm/lib/libLLVMSupport.a
top: /usr/local/opt/llvm/lib/libLLVMIRReader.a
top: /usr/local/opt/llvm/lib/libLLVMMCJIT.a
top: /usr/local/opt/llvm/lib/libLLVMExecutionEngine.a
top: /usr/local/opt/llvm/lib/libLLVMX86CodeGen.a
top: /usr/local/opt/llvm/lib/libLLVMX86AsmParser.a
top: /usr/local/opt/llvm/lib/libLLVMX86Desc.a
top: /usr/local/opt/llvm/lib/libLLVMX86Disassembler.a
top: /usr/local/opt/llvm/lib/libLLVMX86Info.a
top: /usr/local/opt/llvm/lib/libLLVMOrcTargetProcess.a
top: /usr/local/opt/llvm/lib/libLLVMOrcShared.a
top: /usr/local/opt/llvm/lib/libLLVMRuntimeDyld.a
top: /usr/local/opt/llvm/lib/libLLVMAsmPrinter.a
top: /usr/local/opt/llvm/lib/libLLVMCFGuard.a
top: /usr/local/opt/llvm/lib/libLLVMGlobalISel.a
top: /usr/local/opt/llvm/lib/libLLVMIRPrinter.a
top: /usr/local/opt/llvm/lib/libLLVMInstrumentation.a
top: /usr/local/opt/llvm/lib/libLLVMSelectionDAG.a
top: /usr/local/opt/llvm/lib/libLLVMCodeGen.a
top: /usr/local/opt/llvm/lib/libLLVMTarget.a
top: /usr/local/opt/llvm/lib/libLLVMScalarOpts.a
top: /usr/local/opt/llvm/lib/libLLVMAggressiveInstCombine.a
top: /usr/local/opt/llvm/lib/libLLVMInstCombine.a
top: /usr/local/opt/llvm/lib/libLLVMBitWriter.a
top: /usr/local/opt/llvm/lib/libLLVMObjCARCOpts.a
top: /usr/local/opt/llvm/lib/libLLVMTransformUtils.a
top: /usr/local/opt/llvm/lib/libLLVMAnalysis.a
top: /usr/local/opt/llvm/lib/libLLVMProfileData.a
top: /usr/local/opt/llvm/lib/libLLVMSymbolize.a
top: /usr/local/opt/llvm/lib/libLLVMDebugInfoDWARF.a
top: /usr/local/opt/llvm/lib/libLLVMDebugInfoPDB.a
top: /usr/local/opt/llvm/lib/libLLVMObject.a
top: /usr/local/opt/llvm/lib/libLLVMIRReader.a
top: /usr/local/opt/llvm/lib/libLLVMAsmParser.a
top: /usr/local/opt/llvm/lib/libLLVMBitReader.a
top: /usr/local/opt/llvm/lib/libLLVMCore.a
top: /usr/local/opt/llvm/lib/libLLVMRemarks.a
top: /usr/local/opt/llvm/lib/libLLVMBitstreamReader.a
top: /usr/local/opt/llvm/lib/libLLVMTextAPI.a
top: /usr/local/opt/llvm/lib/libLLVMDebugInfoMSF.a
top: /usr/local/opt/llvm/lib/libLLVMDebugInfoBTF.a
top: /usr/local/opt/llvm/lib/libLLVMCodeGenTypes.a
top: /usr/local/opt/llvm/lib/libLLVMMCParser.a
top: /usr/local/opt/llvm/lib/libLLVMMCDisassembler.a
top: /usr/local/opt/llvm/lib/libLLVMMC.a
top: /usr/local/opt/llvm/lib/libLLVMBinaryFormat.a
top: /usr/local/opt/llvm/lib/libLLVMTargetParser.a
top: /usr/local/opt/llvm/lib/libLLVMDebugInfoCodeView.a
top: /usr/local/opt/llvm/lib/libLLVMSupport.a
top: /usr/local/opt/llvm/lib/libLLVMDemangle.a
top: /usr/local/lib/libz3.dylib
top: /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX15.0.sdk/usr/lib/libz.tbd
top: /usr/local/lib/libzstd.dylib
top: CMakeFiles/top.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/Users/taufiksoleh/Personal/Compiler/toplang/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Linking CXX executable top"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/top.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/top.dir/build: top
.PHONY : CMakeFiles/top.dir/build

CMakeFiles/top.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/top.dir/cmake_clean.cmake
.PHONY : CMakeFiles/top.dir/clean

CMakeFiles/top.dir/depend:
	cd /Users/taufiksoleh/Personal/Compiler/toplang/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/taufiksoleh/Personal/Compiler/toplang /Users/taufiksoleh/Personal/Compiler/toplang /Users/taufiksoleh/Personal/Compiler/toplang/build /Users/taufiksoleh/Personal/Compiler/toplang/build /Users/taufiksoleh/Personal/Compiler/toplang/build/CMakeFiles/top.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/top.dir/depend

