package sic.asm.code;

import sic.asm.mnemonics.Mnemonic;

public class Directive extends Node {
    public static final int NOBASE = 1;
    public static final int LTORG = 2;
    public static final int START = 3;
    public static final int END = 4;
    public static final int BASE = 5;
    public static final int EQU = 6;
    public static final int ORG = 7;

    // For directives with numeric operands or symbols
    public int value;
    public String symbol;

    public Directive(Mnemonic mnemonic, int value) {
        super(mnemonic);
        //System.out.println("Created directive:" + mnemonic.name + "value: " + value);
        this.value = value;
        this.symbol = null;
    }

    public Directive(Mnemonic mnemonic, String symbol) {
        super(mnemonic);
        //System.out.println("Created directive:" + mnemonic.name + " symbol: " + symbol + " label:" + label);
        this.symbol = symbol;
        this.value = 0;  // Default value when using symbol
    }

//    @Override
//    public String toString() {
//        if (symbol != null) {
//            return mnemonic.toString() + " " + symbol;
//        } else if (mnemonic.opcode == NOBASE || mnemonic.opcode == LTORG) {
//            return mnemonic.toString();
//        } else {
//            return mnemonic.toString() + " " + value;
//        }
//    }

    @Override
    public void emitCode(byte[] bc) {

    }
    @Override
    public String emitText(byte[] bc, int start, int len) {
        return "";
    }

    @Override
    public String operandToString() {
        if(symbol != null) {
            return symbol;
        }else{
            return Integer.toString(value);
        }
    }

    @Override
    public int length() {
        if (mnemonic.opcode == START) {
            return value;
        }
        return 0;
    }

    @Override
    public void resolve(Code code) {
        super.resolve(code);
    }


    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof Directive)) return false;
        Directive other = (Directive) obj;
        if (symbol != null) {
            return mnemonic.equals(other.mnemonic) && symbol.equals(other.symbol);
        }
        return mnemonic.equals(other.mnemonic) && value == other.value;
    }

    @Override
    public int hashCode() {
        int hash = 7;
        hash = 31 * hash + mnemonic.hashCode();
        hash = 31 * hash + value;
        hash = 31 * hash + (symbol != null ? symbol.hashCode() : 0);
        return hash;
    }
    @Override
    public void enter(Code code) {
        if (mnemonic.opcode == START){
            code.start = value;

        }else if (mnemonic.opcode == EQU){
            value = code.nextLoc;
        }else {
            code.loc = code.nextLoc;
            code.nextLoc += length();
        }
    }
}
