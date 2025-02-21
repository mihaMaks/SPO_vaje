package sic.asm.mnemonics;

import sic.asm.code.*;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;

public class MnemonicF3 extends Mnemonic {

    public MnemonicF3(String name, int opcode, String hint, String desc) {
        super(name, opcode, hint, desc);
    }

    @Override
    public Node parse(Parser parser) throws SyntaxError {
        if (opcode == Opcode.RSUB){
            return new InstructionF3(this);
        }
        char type='0';
        //System.out.println("Peek: " + parser.lexer.peek());

        if (parser.lexer.peek() == '#' || parser.lexer.peek() == '@'){
            type = (parser.lexer.peek()); parser.lexer.advance();
        }

        // number
        if (Character.isDigit(parser.lexer.peek())) {
            int number = parser.parseNumber(0, Code.MAX_WORD);
            if (parser.parseIndexed())
                return new InstructionF3(this, number, 'X');
            return new InstructionF3(this, number, type);
        }
        // symbol
        else if (Character.isLetter(parser.lexer.peek()))
            return new InstructionF3(this, parser.parseSymbol(), type);

        // otherwise: error
        else
            throw new SyntaxError(String.format("Invalid character '%c", parser.lexer.peek()), parser.lexer.row, parser.lexer.col);
    }


    @Override
    public String operandToString(Node instruction) {
        return instruction.toString();
    }
}
