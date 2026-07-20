// SAFE alternative: SAX parser with entity resolution blocked
import * as libxml from 'libxmljs';

const options: libxml.XMLParseOptions = {
  noent: false,
  dtdload: false,
  dtdvalid: false,
  dtdattr: false,
  recover: true,
};

function safeParseXml(xml: string): libxml.XMLDocument {
  return libxml.parseXml(xml, options);
}
