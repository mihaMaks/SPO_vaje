package sic.asm.code;

import sic.asm.mnemonics.Mnemonic;

import java.nio.ByteBuffer;

/**
 * Abstract class Node.
 * Includes label, mnemonic and comment.
 * Podporni razred za predmet Sistemska programska oprema.
 * @author jure
 */
public abstract class Node {

	protected String label;
	protected Mnemonic mnemonic;
	protected String comment;
	protected int location;
	protected int addres;
	protected int opcode;

	public Node(Mnemonic mnemonic) {
		this.mnemonic = mnemonic;
		if (mnemonic == null) {
			//System.out.println("Mnemonic is null");
		}
	}

	public String getLabel() {
		return label == null ? "" : label;
	}

	public void setLabel(String label) {
		this.label = label;
	}

	/**
	 * Return comment as a string.
	 */
	public String getComment() {
		return comment == null ? "" : comment;
	}

	public void setComment(String comment) {
		this.comment = comment;
	}

	/**
	 * Return string representation of the node.
	 * Label and comment are not included.
	 */
	@Override
	public String toString() {
		if(mnemonic.opcode != Directive.EQU)
			return String.format("%06X %06X %-6s %-6s %s", location, opcode, label, mnemonic.toString(), operandToString());
		else
			return String.format("%-4s %-6s %-6s %s", location, label, mnemonic.toString(), comment);
		//return location + " " + mnemonic.toString() + " " + operandToString();
	}

	public String operandToString() {
		return mnemonic.operandToString(this);
	}
	public void enter(Code code){
		code.loc = code.nextLoc;
		code.nextLoc += length();
	}
	public void activate(Code code) {
		location = code.loc;
		code.labels.put(this.label, this.location);
	}
	public void resolve(Code code) {

	}
	public void emitCode(byte[] bc){
		//System.out.println("opco: " + Integer.toHexString(opcode));
		for (int i=location; i<location+length(); i++){
			int m = location+length()-i-1;
			//System.out.print("opcode >>: " + (opcode >> (m)*8) + "|| ");
			bc[i] = (byte) ((opcode >> (m)*8) & 0xFF);

		}
	}
	public String emitText(byte[] bc, int start, int len){
		String bytes = "";
		for (int i=start; i<start+len; i++){
			bytes += String.format("%02X", bc[i]);
		}
//		return bytes + " ";
		return bytes;
	}

	public void leave(Code code){}
	public int length(){
		//return 3*8;
		return 3;
	}


}
