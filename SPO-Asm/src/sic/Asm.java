package sic;

import java.io.*;

import sic.asm.code.Code;
import sic.asm.code.SemanticError;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;

/**
 * Podporni razred za predmet Sistemska programska oprema.
 * @author jure
 */
public class Asm {

	public static String readFile(File file) {
	    byte[] buf = new byte[(int) file.length()];
	    try {
	    	InputStream s = new FileInputStream(file);
	    	try {
	    		s.read(buf);
			} finally {
	    		s.close();
	    	}
    	} catch (IOException e) {
    		return "";
	    }
	    return new String(buf);
	}


	public static void main(String[] args) {
		String input;

		// TODO
		//input = readFile(new File(args[0]));
//		String file_name = "./../../../dn1/arith.asm";
//		String file_name = "./../../../dn1/arithr.asm";
//		String file_name = "./../../../dn1/cat.asm";
//		String file_name = "./../../../dn1/echo.asm";
//		String file_name = "./../../../dn1/poly.asm";
//		String file_name = "./../../../dn1/rec.asm";
//		String file_name = "./../../../dn1/screen.asm";
//		String file_name = "./../../../dn1/ball.asm";
		String file_name = "./../../../dn1/ftf.asm";

		// String workingDir = System.getProperty("user.dir");
    	// System.out.println("Current working directory: " + workingDir);
		input = readFile(new File(file_name));
		file_name = "my_" + file_name.substring(file_name.lastIndexOf('/')+1, file_name.lastIndexOf('.')) + ".obj";
		System.out.println(input);
		Parser parser = new Parser();
		Code code;
		try {
			code = parser.parse(input);
			//code.printLabels();
			code.resolve();
			//code.print();
			code.emitCode();
			System.out.println(code.emitText(code.byteCode));

			try (FileWriter writer = new FileWriter(file_name)) {
				writer.write(code.emitText(code.byteCode));
			} catch (IOException e) {
				e.printStackTrace();
			}

		} catch (SyntaxError e) {
			System.err.println(e);
			System.exit(1);
			return;
		} catch (SemanticError e) {
			System.err.println(e);
			System.exit(1);
			return;
		}
	}

}
