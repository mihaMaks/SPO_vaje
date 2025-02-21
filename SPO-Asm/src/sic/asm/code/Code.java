package sic.asm.code;

import java.nio.ByteBuffer;
import java.util.*;
import sic.asm.code.Node;
/**
 * Podporni razred za predmet Sistemska programska oprema.
 * @author jure
 */
public class Code {
    public static final int MAX_WORD = Integer.MAX_VALUE;
    public static final int MAX_ADDR = 1001;
    public List<Node> code;
    public int base = 0;

    public String name;
    public int start;
    public int length;
    public int end;
    public int loc;
    public int nextLoc;
    public HashMap<String, Integer> labels = new HashMap<String, Integer>();
    public byte[] byteCode;

    public Code() {
        this.code = new ArrayList<Node>();
    }

    public void append(Node n) {
        n.enter(this);
        n.activate(this);
        n.leave(this);
//        System.out.println(n);
        this.code.add(n);
    }
    // TODO
    public void print(){
        for (Node n : this.code) {
            System.out.println(n);
        }
    }
    public void printLabels(){
        if (labels == null || labels.isEmpty()) {
            System.out.println("The HashMap is empty or null.");
            return;
        }

        System.out.println("Contents of the HashMap:");
        for (Map.Entry<String, Integer> entry : labels.entrySet()) {
            System.out.println("Key: " + entry.getKey() + ", Value: " + entry.getValue());
        }
    }

    public void begin(){
        start = 0;
        loc = start; nextLoc = start;
    }
    public void resolve() throws SemanticError {
        begin();
        for (Node node : code) {
            //System.out.println(node);

            node.enter(this);
            node.resolve(this);
            node.leave(this);
            //System.out.println(node);

        }
        length = nextLoc - start;

        System.out.println("length: "+length);
        end();
    }
    public byte[] emitCode(){
        byte[] bc = new byte[length];
        //ByteBuffer bb = ByteBuffer.allocate(length+3*15);
        for (Node node : code) {
            node.emitCode(bc);
            System.out.println(node);
        }
//        for (int i=0; i<bc.length; i++) {
//            System.out.print(String.format("%02X, ", bc[i]));
//        }
        // TO DO
        // napisi emitText()ne bi smel bit tezko
        this.byteCode = bc;
        return bc;
    }
    public String emitText(byte[] bc){
        StringBuilder sb = new StringBuilder();
        // header first
        Node n = code.getFirst();
        sb.append(String.format("H%-6s%06X%06X", n.label, n.length(), length));
//        sb.append(String.format("H %-6s %06X %06X", n.label, n.length(), length));
        StringBuilder lsb = new StringBuilder();
        int start = n.length();
        int endInstructions = 0;
        int endLine = Math.min(length, start + 30);
        boolean newLine = false;
        int procesedBytes = 0;
        for (int i=1; i<code.size(); i++){
            n = code.get(i);
            if(n.mnemonic == null) continue;
            if (!(n.mnemonic.opcode == Storage.RESB || n.mnemonic.opcode == Storage.RESW)){
                if (newLine){
                    sb.append(String.format("\nT%06X%02X", start, endInstructions - start));
//                     sb.append(String.format("\nT %06X %02X ", start, endInstructions - start));
                    start = endLine;
                    endLine = Math.min(start + 30, length);
                    sb.append(lsb);
                    lsb = new StringBuilder();
                    newLine = false;
                    if (n.location + n.length() <= endLine){
                        lsb.append(n.emitText(bc, n.location + procesedBytes, n.length()-procesedBytes));
                        endInstructions = n.location+n.length();
                    }else{
                        lsb.append(n.emitText(bc, n.location + procesedBytes, endLine - n.location-procesedBytes));
                        procesedBytes = endLine - n.location;
                        newLine = true;
                        i--; endInstructions = endLine;
                    }
                    procesedBytes = 0;
                }else{
                    if (n.location + n.length() <= endLine){
                        lsb.append(n.emitText(bc, n.location, n.length()));
                        endInstructions = n.location+n.length();
                    }else{
                        lsb.append(n.emitText(bc, n.location, endLine - n.location));
                        procesedBytes = endLine - n.location;
                        newLine = true;
                        i--; endInstructions = endLine;
                    }

                }
            }else{
                if (!newLine){
                    endLine = n.location; newLine = true;

                }else{
                    endLine = n.location+n.length();

                }
            }
        }
        if (newLine){
        sb.append(String.format("\nT%06X%02X", start, endInstructions - start));
//            sb.append(String.format("\nT %06X %02X ", start, endInstructions - start));
//            System.out.println(newLine);
        }else{
          sb.append(String.format("\nT%06X%02X", start, endLine - start));
//            sb.append(String.format("\nT %06X %02X ", start, endLine - start));
//            System.out.println(newLine);

        }
        sb.append(lsb);
        sb.append(String.format("\nE%06X\n", 0));
//        sb.append(String.format("\nE %06X\n", 0));


        return sb.toString();
    }


    public void end(){}
}
