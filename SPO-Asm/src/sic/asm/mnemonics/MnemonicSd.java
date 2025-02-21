package sic.asm.mnemonics;

import sic.asm.code.Code;
import sic.asm.code.InstructionF2;
import sic.asm.code.Opcode;
import sic.asm.code.Storage;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;

public class MnemonicSd extends Mnemonic {
    byte[] data;
    int value;
    public MnemonicSd(String name, int opcode, String hint, String desc) {
        super(name, opcode, hint, desc);
    }

    @Override
    public Storage parse(Parser parser) throws SyntaxError {
        if (Character.isLetter(parser.lexer.peek())) {
            data = parser.parseData();
            return new Storage(this, data);
        }
        if(Character.isDigit(parser.lexer.peek())) {
            value = parser.parseNumber(0, Code.MAX_WORD);
            return new Storage(this, value);
        }
        else
            throw new SyntaxError(String.format("Invalid character '%c", parser.lexer.peek()), parser.lexer.row, parser.lexer.col);
    }
}
