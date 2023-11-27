<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetCountryByCountryCode</name>
   <tag></tag>
   <elementGuidId>75c61709-6937-4004-bc4c-65ab7568e744</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/xml</value>
      <webElementGuid>50dfcd20-8c6e-4722-b8be-ac1195edabc6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://www.webservicex.net/country.asmx</soapServiceEndpoint>
   <soapServiceFunction>GetISD</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>https://www.swi-prolog.org/pack/file_details/wsdl/examples/country.wsdl</wsdlAddress>
</WebServiceRequestEntity>
