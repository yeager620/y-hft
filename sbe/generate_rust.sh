#!/bin/sh
java -Dsbe.generate.ir=true -Dsbe.target.language=rust -Dsbe.target.namespace=deribit_multicast -Dsbe.output.dir=rust -jar sbe-all-1.28.3.jar deribit_multicast_noheader.xml
