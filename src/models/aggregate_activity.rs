/*
 * Gong API
 *
 * <h2>Overview</h2> <p> The Gong API allows you to: </p> <ol> <li> Receive the following information from Gong: <ol type=\"a\"> <li> Your company's <a href=\"#tag--Calls\">calls</a> in Gong </li> <li> Your company's <a href=\"#tag--Users\">users</a> in Gong </li> <li> Your company's user <a href=\"#tag--Stats\">stats</a> in Gong </li> <li> Your company's user <a href=\"#tag--Settings\">settings</a> in Gong </li> <li> Your company's <a href=\"#tag--Library\">libraries</a> in Gong </li> </ol></li> <li> <a href=\"#post-/v2/calls\">Upload</a> new or  <a href=\"#put-/v2/calls/-id-/media\">update</a>  call recordings in Gong, in order to support cases where you have an internal system that records  calls or obtains them from a third-party entity. </li> <li> <a href=\"#post-/v2/data-privacy/erase-data-for-email-address\">Data Privacy</a>:  Delete users and all their associated elements.</li> <li> Upload <a href=\"#tag--CRM\">CRM</a> data into Gong.  </li> </ol> <p>Check <a href=\"https://app.gong.io/company/api-authentication?currentTab=MY_API_TAB\">here</a> what's your base URL for all API calls. </p> <h2>Authentication</h2>  <p> There are two ways to retrieve credentials to the Gong Public API: </p> <ol><li>Retrieve Manually:<br> <p> In the <a href=\"https://app.gong.io/company/api\">Gong API Page</a> (you must be a technical administrator in Gong), click \"Create\" to receive an <b>Access Key</b>  and an <b>Access Key Secret</b>.<br> </p> <p> Use the Basic Authorization HTTP header (as per <a target=\"_blank\" href=\"https://www.rfc-editor.org/rfc/rfc7617.txt\">RFC</a>) to access the Public API as shown below:<br> <code>Authorization: Basic &lt;token&gt;</code><br> </p> <p> To create the basic token, combine the <b>Access Key</b> and the <b>Access Key Secret</b> with  colon (:) and then encode in Base64 as following:<br> <code>Base64(&lt;accessKey&gt; : &lt;accessKeySecret&gt;)</code><br><br> </p></li> <li>Retrieve through OAuth<br> <p> To obtain the Bearer token, follow the steps described in the <a target=\"_blank\" href=\"https://help.gong.io/hc/en-us/articles/13944551222157-Create-an-app-for-Gong\">Gong OAuth Guide</a>. <br></p> <p> After obtaining the token, use the Bearer Authorization HTTP header (as per <a target=\"_blank\" href=\"https://www.rfc-editor.org/rfc/rfc6750.txt\">RFC</a>) to access the Public API as shown below:<br> <code>Authorization: Bearer &lt;token&gt;</code> </p> </li></ol> <h2>Limits</h2>  <p> By default Gong limits your company's access to the service to 3 API calls per second, and 10,000 API calls per day. </p> <p> When the rate of API calls exceeds these limits an HTTP status code <b>429</b> is returned and a <b>Retry-After</b> header indicates  how many seconds to wait before making a new request. </p><p> If required, contact help@gong.io to change these limits. </p>  <h2>Cursors</h2>  <p> Some API calls that return a list are limited in the amount of records they may return, so multiple API calls may be  required to bring all records. Such an API call also returns a <b>records</b> field, which contains the number of records  in the current page, the current page number and the total number of records. </p> <p> In cases where the total number of records exceeds the number of records thus far retrieved, the <b>records</b> field will also  contain a <b>cursor</b> field which can be used to access the next page of records. To retrieve the next page, repeat the API call with  the <b>cursor</b> value as supplied by the previous API call. All other request inputs should remain the same. </p> <h2>Forward Compatibility</h2>  <p> When coding a system to accept Gong data, take into account that Gong may, without prior warning, add fields to the JSON output.  It is recommended to future proof your code so that it disregards all JSON fields you don't actually use.  </p><p></p>
 *
 * The version of the OpenAPI document: V2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AggregateActivity : Aggregated activity for a user within a time period



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateActivity {
    /// The number of recorded calls this user hosted.
    #[serde(rename = "callsAsHost", skip_serializing_if = "Option::is_none")]
    pub calls_as_host: Option<i32>,
    /// The number of recorded calls the user gave feedback on.
    #[serde(rename = "callsGaveFeedback", skip_serializing_if = "Option::is_none")]
    pub calls_gave_feedback: Option<i32>,
    /// The number of recorded calls the user requested feedback on.
    #[serde(rename = "callsRequestedFeedback", skip_serializing_if = "Option::is_none")]
    pub calls_requested_feedback: Option<i32>,
    /// The number of recorded calls the user received feedback on.
    #[serde(rename = "callsReceivedFeedback", skip_serializing_if = "Option::is_none")]
    pub calls_received_feedback: Option<i32>,
    /// The number of the user's own calls the user listened to.
    #[serde(rename = "ownCallsListenedTo", skip_serializing_if = "Option::is_none")]
    pub own_calls_listened_to: Option<i32>,
    /// The number of other users' calls the user listened to.
    #[serde(rename = "othersCallsListenedTo", skip_serializing_if = "Option::is_none")]
    pub others_calls_listened_to: Option<i32>,
    /// The number of calls the user shared with others inside the company.
    #[serde(rename = "callsSharedInternally", skip_serializing_if = "Option::is_none")]
    pub calls_shared_internally: Option<i32>,
    /// The number of calls the user shared with others outside the company.
    #[serde(rename = "callsSharedExternally", skip_serializing_if = "Option::is_none")]
    pub calls_shared_externally: Option<i32>,
    /// The number of scorecards the user completed.
    #[serde(rename = "callsScorecardsFilled", skip_serializing_if = "Option::is_none")]
    pub calls_scorecards_filled: Option<i32>,
    /// The number of calls in which someone filled a scorecard on the user's calls.
    #[serde(rename = "callsScorecardsReceived", skip_serializing_if = "Option::is_none")]
    pub calls_scorecards_received: Option<i32>,
    /// The number of calls in which this user is participant (not host).
    #[serde(rename = "callsAttended", skip_serializing_if = "Option::is_none")]
    pub calls_attended: Option<i32>,
    /// The number of calls in which a user gave at least one comment.
    #[serde(rename = "callsCommentsGiven", skip_serializing_if = "Option::is_none")]
    pub calls_comments_given: Option<i32>,
    /// The number of calls in which a user received at least one comment on the users calls.
    #[serde(rename = "callsCommentsReceived", skip_serializing_if = "Option::is_none")]
    pub calls_comments_received: Option<i32>,
    /// The number of calls in which someone pressed the \"Mark as reviewed\".
    #[serde(rename = "callsMarkedAsFeedbackGiven", skip_serializing_if = "Option::is_none")]
    pub calls_marked_as_feedback_given: Option<i32>,
    /// The number of calls in which someone pressed the “Mark as reviewed” on the users calls.
    #[serde(rename = "callsMarkedAsFeedbackReceived", skip_serializing_if = "Option::is_none")]
    pub calls_marked_as_feedback_received: Option<i32>,
}

impl AggregateActivity {
    /// Aggregated activity for a user within a time period
    pub fn new() -> AggregateActivity {
        AggregateActivity {
            calls_as_host: None,
            calls_gave_feedback: None,
            calls_requested_feedback: None,
            calls_received_feedback: None,
            own_calls_listened_to: None,
            others_calls_listened_to: None,
            calls_shared_internally: None,
            calls_shared_externally: None,
            calls_scorecards_filled: None,
            calls_scorecards_received: None,
            calls_attended: None,
            calls_comments_given: None,
            calls_comments_received: None,
            calls_marked_as_feedback_given: None,
            calls_marked_as_feedback_received: None,
        }
    }
}


