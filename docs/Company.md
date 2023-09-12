# Company

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | Option<**String**> | The company ID in the partner's source system | [optional]
**name** | Option<**String**> | The name of the company | [optional]
**domain** | Option<**String**> | The domain of the person's company. Used to associate the person with the company (\"acme.com\"). Mandatory for anonymous person, when the name, email, phoneNumber and personBusinessContext are empty | [optional]
**company_business_contexts** | Option<[**Vec<crate::models::CompanyBusinessContext>**](CompanyBusinessContext.md)> | The company's details in an external system such as the CRM. If the domain is empty, to enable association, send the crmAccountId or crmOpportunityId | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


