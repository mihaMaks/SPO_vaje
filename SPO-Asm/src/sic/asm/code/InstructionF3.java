package sic.asm.code;

import sic.asm.mnemonics.Mnemonic;

import java.nio.ByteBuffer;

public class InstructionF3 extends Node{
    public int value;
    public String symbol;
    public char type;
    public int nixbpe;

    public InstructionF3(Mnemonic mnemonic, int value, char type) {
        super(mnemonic);
        this.value = value;
        this.symbol = null;
        this.type = type;

    }
    public InstructionF3(Mnemonic mnemonic) {
        super(mnemonic);
    }

    public InstructionF3(Mnemonic mnemonic, String symbol, char type) {
        super(mnemonic);
        this.value = 0;
        this.symbol = symbol;
        this.type = type;
    }

    @Override
    public void resolve(Code code) {
        super.resolve(code);
        nixbpe = 0x032000; //n=1 i=1 x=0 ...
        if (type == '#'){
            if (symbol != null){
                nixbpe = nixbpe & 0x012000;
            }else {
                nixbpe = nixbpe & 0x010000;
            }
        } else if (type == '@') {
            nixbpe = nixbpe & 0x030000;
        }else if (type == 'X') {
            nixbpe = 0b0000_0011_1000_0000_0000_0000;
        }
        // 0000 0011 0010 0000 0000 0000
        // 0101 0111 1000 0000 0000 0000
        // 0101 0111
        //System.out.println(symbol + " addr: " + code.labels.get(symbol));
        if (symbol != null) {
            addres = code.labels.get(symbol) - this.location - length();
            if (addres < 0) {
                addres = 0xFFF + addres + 1;
            }
        }else{
            addres = value;
            if (addres < 0) {
                addres = 0xFFF + addres + 1;
            }
        }
        //System.out.println(mnemonic.name + " l_addr: " + addres);

    }
    @Override
    public void emitCode(byte[] bc){
        opcode = 0x000000;
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        // set instruction bytes
        opcode = opcode | ((mnemonic.opcode << 16) & 0xFC0000);
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        // set nixbpe
        opcode = opcode | nixbpe;
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        if (mnemonic.opcode != Opcode.RSUB) {
            opcode = opcode | addres;
        }else{
            opcode = opcode & 0xFF0000;
        }
        //System.out.println("instruction: " + mnemonic.name + " opcode: " + Integer.toHexString(opcode));
        //System.out.println("ukaz: " + mnemonic.name + " pise a mesto: "+ location);
//        for (int i=location; i<location+length(); i++){
//            int m = location - i;
//            byteCode[i] = (byte) ((opcode >> m*8) & 0xFF);
//        }
        super.emitCode(bc);
    }

    @Override
    public String operandToString() {
        if (symbol != null) {
            return symbol + " = " + addres ;
        } else {
            return Integer.toString(value);
        }
    }
}
