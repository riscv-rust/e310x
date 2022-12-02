/* We have to include previous .x files and their dependencies */
INCLUDE hifive1-link.x

/* Declare the external handlers array and the default handler as external symbols */
EXTERN(HANDLERS);
EXTERN(DefaultMachineExternal);
/* Provide the weak symbols for each interrupt handler */
PROVIDE(GPIO4 = DefaultMachineExternal);
