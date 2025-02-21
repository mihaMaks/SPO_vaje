package sic.asm.code;


import sic.asm.mnemonics.Mnemonic;
import sic.asm.mnemonics.MnemonicF2r;
import sic.asm.mnemonics.MnemonicF2rn;
import sic.asm.mnemonics.MnemonicF2rr;

import java.nio.ByteBuffer;


public class InstructionF2 extends Node {
    int reg1, reg2;
    String registers = "AXLBSTF";

    public InstructionF2(Mnemonic mnemonic, int reg1, int reg2) {
        super(mnemonic);
        this.reg1 = reg1;
        this.reg2 = reg2;
    }
    public InstructionF2(Mnemonic mnemonic, int reg1) {
        super(mnemonic);
        this.reg1 = reg1;
    }
    @Override
    public void emitCode(byte[] bc){
        opcode = 0x0000;
        opcode = opcode + (mnemonic.opcode << 8);
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        // set instruction bytes
        opcode = opcode | ((reg1 << 4) | reg2);
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        super.emitCode(bc);
    }

    @Override
    public int length() {
        return 2;
    }

    @Override
    public String operandToString() {
        if (this.mnemonic.getClass() == MnemonicF2rn.class)
            return registers.charAt(getReg1()) + "," + getReg2();
        if (this.mnemonic.getClass() == MnemonicF2rr.class)
            return registers.charAt(getReg1()) + "," + registers.charAt(getReg2());
        if (this.mnemonic.getClass() == MnemonicF2r.class)
            return Integer.toString(this.getReg1());
        else
            return "Error maching mnemonic class.";
    }

    public int getReg1() {
        return reg1;
    }

    public void setReg1(int reg1) {
        this.reg1 = reg1;
    }

    public int getReg2() {
        return reg2;
    }

    public void setReg2(int reg2) {
        this.reg2 = reg2;
    }



}
