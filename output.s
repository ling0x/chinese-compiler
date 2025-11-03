	.file	"chinese_program"
	.text
	.globl	main                            # -- Begin function main
	.p2align	4
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$.Lfmt, %edi
	movl	$".L诗一", %esi
	xorl	%eax, %eax
	callq	printf@PLT
	movl	$.Lfmt.1, %edi
	movl	$".L诗二", %esi
	xorl	%eax, %eax
	callq	printf@PLT
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	".L第一句",@object              # @"\E7\AC\AC\E4\B8\80\E5\8F\A5"
	.section	.rodata.str1.1,"aMS",@progbits,1
".L第一句":
	.asciz	"\351\233\266\346\210\220\346\234\254\346\212\275\350\261\241"
	.size	".L第一句", 16

	.type	".L第二句",@object              # @"\E7\AC\AC\E4\BA\8C\E5\8F\A5"
".L第二句":
	.asciz	"\346\200\247\350\203\275\346\227\240\345\217\214\347\232\204Rust\350\257\255\350\250\200"
	.size	".L第二句", 26

	.type	".L诗一",@object                # @"\E8\AF\97\E4\B8\80"
".L诗一":
	.asciz	"$\347\254\254\344\270\200\345\217\245, $\347\254\254\344\272\214\345\217\245."
	.size	".L诗一", 24

	.type	.Lfmt,@object                   # @fmt
.Lfmt:
	.asciz	"%s\n"
	.size	.Lfmt, 4

	.type	".L第三句",@object              # @"\E7\AC\AC\E4\B8\89\E5\8F\A5"
".L第三句":
	.asciz	"\346\227\240\345\236\203\345\234\276\345\233\236\346\224\266"
	.size	".L第三句", 16

	.type	".L第四句",@object              # @"\E7\AC\AC\E5\9B\9B\E5\8F\A5"
".L第四句":
	.asciz	"\345\215\264\346\227\240\345\206\205\345\255\230\346\263\204\346\274\217"
	.size	".L第四句", 19

	.type	".L第五句",@object              # @"\E7\AC\AC\E4\BA\94\E5\8F\A5"
".L第五句":
	.asciz	"\344\273\216\347\263\273\347\273\237\345\272\225\345\261\202\345\210\260\347\275\221\347\273\234\346\234\215\345\212\241"
	.size	".L第五句", 31

	.type	".L第六句",@object              # @"\E7\AC\AC\E5\85\AD\E5\8F\A5"
".L第六句":
	.asciz	"Rust\350\257\255\350\250\200"
	.size	".L第六句", 11

	.type	".L第七句",@object              # @"\E7\AC\AC\E4\B8\83\E5\8F\A5"
".L第七句":
	.asciz	"\347\216\260\344\273\243\347\263\273\347\273\237\347\274\226\347\250\213\347\232\204\346\234\252\346\235\245"
	.size	".L第七句", 28

	.type	".L诗二",@object                # @"\E8\AF\97\E4\BA\8C"
".L诗二":
	.asciz	"$\347\254\254\344\270\211\345\217\245, $\347\254\254\345\233\233\345\217\245, $\347\254\254\344\272\224\345\217\245, $\347\254\254\345\205\255\345\217\245, $\347\254\254\344\270\203\345\217\245."
	.size	".L诗二", 60

	.type	.Lfmt.1,@object                 # @fmt.1
.Lfmt.1:
	.asciz	"%s\n"
	.size	.Lfmt.1, 4

	.section	".note.GNU-stack","",@progbits
