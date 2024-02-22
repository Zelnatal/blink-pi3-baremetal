ENTRY(_start)

SECTIONS
{
    . = 0x200000;
    .start :
    {
        *(.start._start)
    }
}