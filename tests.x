SECTIONS
{
  .riscv_tests : ALIGN(4)
  {
    __riscv_tests_start = .;
    KEEP(*(SORT_BY_NAME(.riscv_tests.*)));
    __riscv_tests_end = .;
  } > REGION_RODATA
}
INSERT AFTER .rodata;
