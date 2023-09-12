# NewCallAddingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_unique_id** | **String** | A call's unique identifier in the PBX or the recording system. Gong uses this identifier to prevent repeated attempts to upload the same recording. | 
**title** | Option<**String**> | The title of the call. This title is available in the Gong system for indexing and search. | [optional]
**purpose** | Option<**String**> | The purpose of the call. This optional field is a free text of up to 255 characters. | [optional]
**scheduled_start** | Option<**String**> | The date and time the call was scheduled to begin in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); | [optional]
**scheduled_end** | Option<**String**> | The date and time the call was scheduled to end in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); | [optional]
**actual_start** | **String** | The actual date and time when the call started in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); | 
**duration** | Option<**f32**> | The actual call duration in seconds. | [optional]
**parties** | [**Vec<crate::models::CallParticipant>**](CallParticipant.md) | A list of the call's participants. A party must be provided for the primaryUser. | 
**direction** | **String** | Whether the call is inbound (someone called the company), outbound (a rep dialed someone outside the company), or a conference call. | 
**disposition** | Option<**String**> | The disposition of the call. The disposition is free text of up to 255 characters. | [optional]
**context** | Option<[**Vec<crate::models::CallUploadContext>**](CallUploadContext.md)> | A list of references to external systems such as CRM, Telephony System, Case Management, etc. | [optional]
**custom_data** | Option<**String**> | Optional metadata associated with the call (represented as text). Gong stores this metadata and it can be used for troubleshooting. | [optional]
**speakers_timeline** | Option<[**crate::models::SpeakersTimeline**](SpeakersTimeline.md)> |  | [optional]
**meeting_url** | Option<**String**> | The URL of the conference call by which users join the meeting | [optional]
**call_provider_code** | Option<**String**> | The code identifies the provider conferencing or telephony system. For example: zoom, clearslide, gotomeeting, ringcentral, outreach, insidesales, etc. These values are predefined by Gong, please contact help@gong.io to find the proper value for your system. | [optional]
**download_media_url** | Option<**String**> | The URL from which Gong can download the media file. The URL must be unique, the audio or video file must be a maximum of 1.5GB. If you provide this URL, you should not perform the 'Add call media' step. | [optional]
**workspace_id** | Option<**String**> | Optional workspace identifier. If specified, the call will be placed into this workspace, otherwise, the default algorithm for workspace placement will be applied. | [optional]
**language_code** | Option<**String**> | The language code the call should be transcribed to. This field is optional as Gong automatically detects the language spoken in the call and transcribes it accordingly. Set this field only if you are sure of the language the call is in. Valid values are: af-ZA, am-ET, ar-AE, ar-BH, ar-DZ, ar-EG, ar-IL, ar-IQ, ar-JO, ar-KW, ar-LB, ar-MA, ar-MR, ar-OM, ar-PS, ar-QA, ar-SA, ar-TN, ar-YE, az-AZ, bg-BG, bn-BD, bn-IN, bs-BA, ca-ES, cs-CZ, da-DK, de-AT, de-CH, de-DE, el-GR, en-AB, en-AU, en-CA, en-GB, en-IE, en-IN, en-NZ, en-PH, en-SG, en-US, en-WL, en-ZA, es-AR, es-BO, es-CL, es-CO, es-CR, es-DO, es-EC, es-ES, es-GT, es-HN, es-MX, es-NI, es-PA, es-PE, es-PR, es-PY, es-SV, es-US, es-UY, et-EE, eu-ES, fa-IR, fi-FI, fil-PH, fr-BE, fr-CA, fr-CH, fr-FR, gl-ES, gu-IN, he-IL, hi-IN, hr-HR, hu-HU, hy-AM, id-ID, is-IS, it-CH, it-IT, ja-JP, jv-ID, ka-GE, kk-KZ, km-KH, kn-IN, ko-KR, lo-LA, lt-LT, lv-LV, mk-MK, ml-IN, mn-MN, mr-IN, ms-MY, my-MM, ne-NP, nl-BE, nl-NL, no-NO, pa-Guru-IN, pl-PL, pt-BR, pt-PT, ro-RO, ru-RU, si-LK, sk-SK, sl-SI, sq-AL, sr-RS, su-ID, sv-SE, sw-KE, sw-TZ, ta-IN, ta-LK, ta-MY, ta-SG, te-IN, th-TH, tr-TR, uk-UA, ur-IN, ur-PK, uz-UZ, vi-VN, yue-Hant-HK, zh-CN, zh-TW, zu-ZA | [optional]
**primary_user** | **String** | The Gong internal user ID of the team member who hosted the call. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


