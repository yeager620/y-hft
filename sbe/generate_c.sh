#!/bin/sh
java -Dsbe.generate.ir=true -Dsbe.target.language=c -Dsbe.target.namespace=deribit_multicast -Dsbe.output.dir=c -jar sbe-all-1.28.3.jar deribit_multicast.xml
