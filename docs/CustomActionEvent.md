# CustomActionEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reporting_system** | **String** | The unique identifier of the reporting system. It is the same value in all events originating from the same system. | 
**event_timestamp** | **String** | The date and time when the event happened in the ISO-8601 format (e.g., '2021-08-01T02:30:00+05:00' or '2021-08-01T08:00:00Z', where Z stands for UTC); | 
**event_id** | Option<**String**> | The original id of the event as designated in the reporting system. | [optional]
**content_id** | Option<**String**> | The id of the content that was viewed in the reporting system. | [optional]
**content_url** | Option<**String**> | The url of the content that was viewed in the reporting system. This is the url that is was accessed by the viewer. | [optional]
**content_title** | Option<**String**> | Human readable title of the content. | [optional]
**action_name** | Option<**String**> | The name of the action like \"Document Viewed\" or \"Presentation Opened\". | [optional]
**event_info_url** | Option<**String**> | The link to a page that presents additional information about this event. | [optional]
**actor** | Option<[**crate::models::Actor**](Actor.md)> |  | [optional]
**crm_context** | Option<[**Vec<crate::models::CallUploadContext>**](CallUploadContext.md)> | A list of references to external systems such as CRM, Telephony System, Case Management, etc. | [optional]
**content_properties** | Option<[**Vec<crate::models::GenericProperty>**](Generic Property.md)> | A list of additional properties for the content | [optional]
**event_properties** | Option<[**Vec<crate::models::GenericProperty>**](Generic Property.md)> | A list of additional properties for the event | [optional]
**user_agent** | Option<**String**> | \"User-Agent\" header value for browser based interaction | [optional]
**mobile_app_id** | Option<**String**> | The application identification string in case of interaction via mobile application (bundle identifier or package name). | [optional]
**agent_platform** | Option<**String**> | Platform on which the interaction was made | [optional]
**workspace_id** | Option<**String**> | Optional workspace identifier. If specified, the event will be placed into this workspace, otherwise, the default algorithm for workspace placement will be applied. | [optional]
**non_company_participants** | Option<[**Vec<crate::models::Actor>**](Actor.md)> |  | [optional]
**more_info_url** | Option<**String**> |  | [optional]
**share_id** | Option<**String**> |  | [optional]
**sharer** | Option<[**crate::models::Sharer**](Sharer.md)> |  | [optional]
**sharing_message_body** | Option<**String**> |  | [optional]
**sharing_message_subject** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


