OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS
{
	. = 0x1000;

	.text :
	{
		*(.text .text.*)
		. = ALIGN(4K);
	}

	.data :
	{
		*(.data .data.*)
		. = ALIGN(4K);
	}

	.bss :
	{
		*(.bss .bss.*)
		. = ALIGN(4K);
	}
}