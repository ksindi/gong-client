# Person

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the person | [optional]
**email** | Option<**String**> | The email address of the person. Used for business context association | [optional]
**phone_number** | Option<**String**> | The phone number of the person. Used for business context association | [optional]
**person_id** | Option<**String**> | A unique person identifier in the partner system. Mandatory for anonymous person, when the name, email, phoneNumber and personBusinessContext are empty | [optional]
**person_business_context** | Option<[**crate::models::PersonBusinessContext**](PersonBusinessContext.md)> |  | [optional]
**location** | Option<[**crate::models::Location**](Location.md)> |  | [optional]
**company** | Option<[**crate::models::Company**](Company.md)> |  | [optional]
**person_custom_fields** | Option<[**Vec<crate::models::CustomField>**](CustomField.md)> | Include additional custom data about the person | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


