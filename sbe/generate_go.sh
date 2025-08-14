#!/bin/sh
java -Dsbe.generate.ir=true -Dsbe.target.language=golang -Dsbe.target.namespace=deribit_multicast -Dsbe.output.dir=go -jar sbe-all-1.28.3.jar deribit_multicast.xml
