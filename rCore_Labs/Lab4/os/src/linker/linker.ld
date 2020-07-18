/* Arch */
OUTPUT_ARCH(riscv)

/* start entry */
ENTRY(_start)

/* start address of data */
BASE_ADDRESS = 0xffffffff80200000; /* VirtualAddress */

SECTIONS
{
    /* . is location counter */
    . = BASE_ADDRESS;

    /* start of kernel */
    kernel_start = .;

    /* align */
    . = ALIGN(4K);
    text_start = .;

    /* .text field */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
    }

    /* align */
    . = ALIGN(4K);
    rodata_start = .;

    /* .rodata field */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }

    /* align */
    . = ALIGN(4K);
    data_start = .;

    /* .data field */
    .data : {
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.data .data.*)
    }

    /* align */
    . = ALIGN(4K);
    bss_start = .;

    /* .bss field */
    .bss : {
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
    }

    /* align */
    . = ALIGN(4K);
    /* end of kernel */
    kernel_end = .;
}