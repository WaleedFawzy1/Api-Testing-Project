<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetCountryByCurrencyCode</name>
   <tag></tag>
   <elementGuidId>f5e169dc-0054-433c-8e87-064a6df296c7</elementGuidId>
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
      <value>http://www.webserviceX.NET/GetCountryByCurrencyCode</value>
      <webElementGuid>1425b4cb-9ff1-4066-98ff-2e51bccfb36c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>526cc7f6-44ff-4733-84d8-db8b77bd332d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://www.webserviceX.NET&quot;>&#xd;
   &lt;soapenv:Header/>&#xd;
   &lt;soapenv:Body>&#xd;
      &lt;web:GetCountryByCurrencyCode>&#xd;
         &lt;web:CurrencyCode>gero et&lt;/web:CurrencyCode>&#xd;
      &lt;/web:GetCountryByCurrencyCode>&#xd;
   &lt;/soapenv:Body>&#xd;
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://www.webservicex.net/country.asmx</soapServiceEndpoint>
   <soapServiceFunction>GetCountryByCurrencyCode</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>https://www.swi-prolog.org/pack/file_details/wsdl/examples/country.wsdl</wsdlAddress>
</WebServiceRequestEntity>