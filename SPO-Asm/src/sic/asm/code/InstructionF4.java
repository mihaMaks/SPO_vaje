package sic.asm.code;

import sic.asm.mnemonics.Mnemonic;

import java.nio.ByteBuffer;

public class InstructionF4 extends Node{
    public int value;
    public String symbol;
    public int nixbpe;

    public InstructionF4(Mnemonic mnemonic, int value) {
        super(mnemonic);
        this.value = value;
        this.symbol = null;
    }

    public InstructionF4(Mnemonic mnemonic, String symbol) {
        super(mnemonic);
        this.value = 0;
        this.symbol = symbol;
    }
    @Override
    public void resolve(Code code) {
        super.resolve(code);
        nixbpe = 0x03100000; //n=1 i=1 x=0 ...
        //System.out.println(symbol + " addr: "); //+ code.labels.get(symbol));
        addres = code.labels.get(symbol);
        //System.out.println(mnemonic.name + " l_addr: " + addres);

    }
    @Override
    public void emitCode(byte[] bc){
        opcode = 0x00000000;
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        // set instruction bytes
        opcode = opcode | ((mnemonic.opcode << 24) & 0xFC000000);
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        // set nixbpe
        opcode = opcode | nixbpe;
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        opcode = opcode | addres;
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        super.emitCode(bc);

    }
    @Override
    public String operandToString() {
        if (symbol != null) {
            return symbol;
        } else {
            return Integer.toString(value);
        }
    }

    @Override
    public int length(){
        //return 4*8;
        System.out.println(4);
        return 4;
    }
}
