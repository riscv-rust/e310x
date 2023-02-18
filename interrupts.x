/* This file should be included by hifive1 if virq feature is enabled */

/* Weak symbol for other machine external interrupts handler */
PROVIDE(OtherMachineExternal = DefaultMachineExternal);

/* Weak symbols for each external interrupt handler */
PROVIDE(WATCHDOG = OtherMachineExternal);
PROVIDE(RTC = OtherMachineExternal);
PROVIDE(UART0 = OtherMachineExternal);
PROVIDE(UART1 = OtherMachineExternal);
PROVIDE(QSPI0 = OtherMachineExternal);
PROVIDE(QSPI1 = OtherMachineExternal);
PROVIDE(QSPI2 = OtherMachineExternal);
PROVIDE(GPIO0 = OtherMachineExternal);
PROVIDE(GPIO1 = OtherMachineExternal);
PROVIDE(GPIO2 = OtherMachineExternal);
PROVIDE(GPIO3 = OtherMachineExternal);
PROVIDE(GPIO4 = OtherMachineExternal);
PROVIDE(GPIO5 = OtherMachineExternal);
PROVIDE(GPIO6 = OtherMachineExternal);
PROVIDE(GPIO7 = OtherMachineExternal);
PROVIDE(GPIO8 = OtherMachineExternal);
PROVIDE(GPIO9 = OtherMachineExternal);
PROVIDE(GPIO10 = OtherMachineExternal);
PROVIDE(GPIO11 = OtherMachineExternal);
PROVIDE(GPIO12 = OtherMachineExternal);
PROVIDE(GPIO13 = OtherMachineExternal);
PROVIDE(GPIO14 = OtherMachineExternal);
PROVIDE(GPIO15 = OtherMachineExternal);
PROVIDE(GPIO16 = OtherMachineExternal);
PROVIDE(GPIO17 = OtherMachineExternal);
PROVIDE(GPIO18 = OtherMachineExternal);
PROVIDE(GPIO19 = OtherMachineExternal);
PROVIDE(GPIO20 = OtherMachineExternal);
PROVIDE(GPIO21 = OtherMachineExternal);
PROVIDE(GPIO22 = OtherMachineExternal);
PROVIDE(GPIO23 = OtherMachineExternal);
PROVIDE(GPIO24 = OtherMachineExternal);
PROVIDE(GPIO25 = OtherMachineExternal);
PROVIDE(GPIO26 = OtherMachineExternal);
PROVIDE(GPIO27 = OtherMachineExternal);
PROVIDE(GPIO28 = OtherMachineExternal);
PROVIDE(GPIO29 = OtherMachineExternal);
PROVIDE(GPIO30 = OtherMachineExternal);
PROVIDE(GPIO31 = OtherMachineExternal);
PROVIDE(PWM0CMP0 = OtherMachineExternal);
PROVIDE(PWM0CMP1 = OtherMachineExternal);
PROVIDE(PWM0CMP2 = OtherMachineExternal);
PROVIDE(PWM0CMP3 = OtherMachineExternal);
PROVIDE(PWM1CMP0 = OtherMachineExternal);
PROVIDE(PWM1CMP1 = OtherMachineExternal);
PROVIDE(PWM1CMP2 = OtherMachineExternal);
PROVIDE(PWM1CMP3 = OtherMachineExternal);
PROVIDE(PWM2CMP0 = OtherMachineExternal);
PROVIDE(PWM2CMP1 = OtherMachineExternal);
PROVIDE(PWM2CMP2 = OtherMachineExternal);
PROVIDE(PWM2CMP3 = OtherMachineExternal);
/* Weak symbol for I2C0 (g002 only) */
PROVIDE(I2C0 = OtherMachineExternal);
