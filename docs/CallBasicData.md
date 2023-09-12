# CallBasicData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Gong's unique numeric identifier for the call (up to 20 digits). | [optional]
**url** | Option<**String**> | The URL to the page in the Gong web application where the call is available. | [optional]
**title** | Option<**String**> | The title of the call. | [optional]
**scheduled** | Option<**String**> | Scheduled date and time of the call in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**started** | Option<**String**> | The date and time when the call was recorded in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [optional]
**duration** | Option<**i64**> | The duration of the call, in seconds. | [optional]
**primary_user_id** | Option<**String**> | The primary user ID of the team member who hosted the call. | [optional]
**direction** | Option<**String**> | Call direction. | [optional]
**system** | Option<**String**> | The system with which the call was carried out (e.g., WebEx, ShoreTel, etc.). | [optional]
**scope** | Option<**String**> | The scope of the call: 'internal' if all the participants are from the company, 'external' if some participants are not from the company, or 'unknown' if the scope is unknown. | [optional]
**media** | Option<**String**> | Media type | [optional]
**language** | Option<**String**> | The language codes (as defined by ISO-639-2B). E.g., eng, fre, spa, ger, and ita. Also used are und (unsupported language), and zxx (not enough speech content for identification). | [optional]
**workspace_id** | Option<**String**> | Gong's unique numeric identifier for the call's workspace (up to 20 digits). | [optional]
**sdr_disposition** | Option<**String**> | The SDR disposition of the call | [optional]
**client_unique_id** | Option<**String**> | The call's unique identifier in the origin recording system (typically a telephony recording system). The identifier is provided to Gong during the call creation via the Public API or through telephony systems integrations. | [optional]
**custom_data** | Option<**String**> | Metadata as was provided to Gong during the call creation via the Public API. | [optional]
**purpose** | Option<**String**> | The purpose of the call. | [optional]
**meeting_url** | Option<**String**> | The meeting provider URL on which the web conference was recorded. | [optional]
**is_private** | Option<**bool**> | If the call is private. | [optional]
**calendar_event_id** | Option<**String**> | The Id of the meeting in Google or Outlook Calendar. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


