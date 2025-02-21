package sic.asm.mnemonics;

import sic.asm.code.Node;
import sic.asm.code.Storage;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;


/**
 * Podporni razred za predmet Sistemska programska oprema.
 * @author jure
 */
public abstract class Mnemonic {
	public String name;
	public int opcode;
	public String hint;
	public String desc;

	public Mnemonic(String name, int opcode, String hint, String desc) {
		this.name = name;
		this.opcode = opcode;
		this.hint = hint;
		this.desc = desc;
	}
	
	public abstract Node parse(Parser parser) throws SyntaxError;

	@Override
	public String toString() {
		return String.format(" %-6s", name);
	}

	public String operandToString(Node instruction) {
		return "";
	}
}