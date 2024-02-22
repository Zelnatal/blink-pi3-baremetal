ENTRY(_start)

SECTIONS
{
    . = 0x200000;
    .text :
    {
        *(.text)
    }

    /DISCARD/ :
    {
        *(.comment)
    }
}