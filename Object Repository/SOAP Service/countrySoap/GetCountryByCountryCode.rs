<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetCountryByCountryCode</name>
   <tag></tag>
   <elementGuidId>d48a6b8f-916f-4a5a-89a1-44b59f7576b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://www.webserviceX.NET/GetCountryByCountryCode</value>
      <webElementGuid>946e7a6b-75f4-42d5-9139-6b35f1437e44</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>8ecc7435-b642-4835-8b10-fc4dbcee16f4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://www.webserviceX.NET&quot;>&#xd;
   &lt;soapenv:Header/>&#xd;
   &lt;soapenv:Body>&#xd;
      &lt;web:GetCountryByCountryCode>&#xd;
         &lt;web:CountryCode>gero et&lt;/web:CountryCode>&#xd;
      &lt;/web:GetCountryByCountryCode>&#xd;
   &lt;/soapenv:Body>&#xd;
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://www.webservicex.net/country.asmx</soapServiceEndpoint>
   <soapServiceFunction>GetCountryByCountryCode</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>https://www.swi-prolog.org/pack/file_details/wsdl/examples/country.wsdl</wsdlAddress>
</WebServiceRequestEntity>