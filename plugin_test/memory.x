MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 32M
}

SECTIONS
{

  .text ORIGIN(RAM) :
  {
    *(.text .text.*);
  } > RAM

}