package sic.asm.code;

import sic.asm.mnemonics.Mnemonic;
import sic.asm.mnemonics.MnemonicSd;

import java.util.Arrays;

public class Storage extends Node {

    // Storage directive constants
    public static final int RESB = 1; // Reserve bytes
    public static final int RESW = 2; // Reserve words
    public static final int BYTE = 3; // Initialize bytes
    public static final int WORD = 4; // Initialize words
    public byte[] data;
    public int value;
    public Storage(Mnemonic mnemonic, byte[] data) {
        super(mnemonic);
        this.data = data;
    }
    public Storage(Mnemonic mnemonic, int value) {
        super(mnemonic);
        this.value = value;
    }

    @Override
    public String operandToString() {
        if (this.mnemonic.getClass() == MnemonicSd.class && data != null)
            return Arrays.toString(this.data);
        return Integer.toString(this.value);
    }
    @Override
    public int length() {
        if(data != null)
            return data.length;
        if (this.mnemonic.opcode == RESB) {
            return value;
        }
        else if (this.mnemonic.opcode == RESW) {
            return value * 3;
        }
        else if (this.mnemonic.opcode == BYTE) {
            return 1;
        }
        return 3;
    }
    @Override
    public void emitCode(byte[] bc) {
        if (this.mnemonic.opcode == WORD || this.mnemonic.opcode == BYTE) {
            if (data != null){
                for (int i = location; i < location + length(); i++) {
                    int m = location + length() - i - 1;
                    //System.out.print("opcode >>: " + (opcode >> (m)*8) + "|| ");
                    int a = i - location;
                    if (data[a] < 0){
                        data[a] = (byte) (0xFFF + data[a] +1);
                    }
                    bc[i] = data[a];
                }
            }else {
                for (int i = location; i < location + length(); i++) {
                    int m = location + length() - i - 1;
                    //System.out.print("opcode >>: " + (opcode >> (m)*8) + "|| ");
                    bc[i] = (byte) ((value >> (m) * 8) & 0xFF);
                }
            }
        }


    }



    /**
     * Validates a storage type.
     *
     * @param type the type of the storage directive
     * @return true if the type is valid, false otherwise
     */
    public static boolean isValid(int type) {
        return type == RESB || type == RESW || type == BYTE || type == WORD;
    }
}
