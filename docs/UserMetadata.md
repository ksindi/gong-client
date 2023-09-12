# UserMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Gong's unique numeric identifier for the user (up to 20 digits). | [optional]
**email_address** | Option<**String**> | The email address of the Gong user. | [optional]
**created** | Option<**String**> | Creation time in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC);of the Gong user. | [optional]
**active** | Option<**bool**> | True if the user is active, false if not. | [optional]
**email_aliases** | Option<**Vec<String>**> | List of email address aliases of the Gong user. | [optional]
**trusted_email_address** | Option<**String**> | The trusted authentication email assigned to the Gong user | [optional]
**first_name** | Option<**String**> | The first name of the Gong user. | [optional]
**last_name** | Option<**String**> | The last name of the Gong user. | [optional]
**title** | Option<**String**> | The job title of the Gong user. | [optional]
**phone_number** | Option<**String**> | The phone number of the Gong user. | [optional]
**extension** | Option<**String**> | The extension number of the Gong user. | [optional]
**personal_meeting_urls** | Option<**Vec<String>**> | The list of personal meeting URLs of the Gong user. | [optional]
**settings** | Option<[**crate::models::Settings**](Settings.md)> |  | [optional]
**manager_id** | Option<**String**> | The manager ID of the Gong user. | [optional]
**meeting_consent_page_url** | Option<**String**> | The Gong recording consent meeting link | [optional]
**spoken_languages** | Option<[**Vec<crate::models::SpokenLanguage>**](spokenLanguage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


