# ContentSharedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reporting_system** | **String** | The unique identifier of the reporting system. It is the same value in all events originating from the same system. | 
**event_timestamp** | **String** | The date and time when the event happened in the ISO-8601 format (e.g., '2021-08-01T02:30:00+05:00' or '2021-08-01T08:00:00Z', where Z stands for UTC); | 
**event_id** | Option<**String**> | The original id of the event as designated in the reporting system. | [optional]
**content_id** | **String** | The id of the content that was shared in the reporting system. | 
**content_url** | **String** | The url of the content that was shared in the reporting system. This is the url that is was accessed by the viewer. | 
**content_title** | **String** | Human readable title of the content. | 
**share_id** | Option<**String**> | The id of the share action, in case there can be more than one share per content. | [optional]
**share_info_url** | Option<**String**> | The link to a page that presents additional information about this event. | [optional]
**sharing_message_subject** | Option<**String**> | The subject of share email / message. | [optional]
**sharing_message_body** | Option<**String**> | The share message body. Can contain HTML and will be cleaned when it is presented. | [optional]
**sharer** | Option<[**crate::models::Sharer**](Sharer.md)> |  | [optional]
**recipients** | Option<[**Vec<crate::models::Actor>**](Actor.md)> | array of objects with  name + email of recipients. Email is required. | [optional]
**crm_context** | Option<[**Vec<crate::models::CallUploadContext>**](CallUploadContext.md)> | A list of references to external systems such as CRM, Telephony System, Case Management, etc. | [optional]
**content_properties** | Option<[**Vec<crate::models::GenericProperty>**](Generic Property.md)> | A list of additional properties for the content | [optional]
**event_properties** | Option<[**Vec<crate::models::GenericProperty>**](Generic Property.md)> | A list of additional properties for the event | [optional]
**workspace_id** | Option<**String**> | Optional workspace identifier. If specified, the event will be placed into this workspace, otherwise, the default algorithm for workspace placement will be applied. | [optional]
**action_name** | Option<**String**> | The name of the action like \"Document Sent\" or \"Presentation Shared\". | [optional]
**non_company_participants** | Option<[**Vec<crate::models::Actor>**](Actor.md)> |  | [optional]
**more_info_url** | Option<**String**> |  | [optional]
**mobile_app_id** | Option<**String**> |  | [optional]
**agent_platform** | Option<**String**> |  | [optional]
**user_agent** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


