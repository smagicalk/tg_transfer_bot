#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallProblem {
    /// The user heard their own voice
    #[serde(rename(serialize = "callProblemEcho", deserialize = "callProblemEcho"))]
    Echo,
    /// The user heard background noise
    #[serde(rename(serialize = "callProblemNoise", deserialize = "callProblemNoise"))]
    Noise,
    /// The other side kept disappearing
    #[serde(rename(serialize = "callProblemInterruptions", deserialize = "callProblemInterruptions"))]
    Interruptions,
    /// The speech was distorted
    #[serde(rename(serialize = "callProblemDistortedSpeech", deserialize = "callProblemDistortedSpeech"))]
    DistortedSpeech,
    /// The user couldn't hear the other side
    #[serde(rename(serialize = "callProblemSilentLocal", deserialize = "callProblemSilentLocal"))]
    SilentLocal,
    /// The other side couldn't hear the user
    #[serde(rename(serialize = "callProblemSilentRemote", deserialize = "callProblemSilentRemote"))]
    SilentRemote,
    /// The call ended unexpectedly
    #[serde(rename(serialize = "callProblemDropped", deserialize = "callProblemDropped"))]
    Dropped,
    /// The video was distorted
    #[serde(rename(serialize = "callProblemDistortedVideo", deserialize = "callProblemDistortedVideo"))]
    DistortedVideo,
    /// The video was pixelated
    #[serde(rename(serialize = "callProblemPixelatedVideo", deserialize = "callProblemPixelatedVideo"))]
    PixelatedVideo,
}
