use super::*;



// region CreateProfileRequestProfileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// One or more profile types that the agent is capable of providing.
pub enum CreateProfileRequestProfileTypeEnum {
    

    /// Unspecified profile type.
    ///
    /// "PROFILE_TYPE_UNSPECIFIED"
    #[serde(rename="PROFILE_TYPE_UNSPECIFIED")]
    PROFILETYPEUNSPECIFIED,
    

    /// Thread CPU time sampling.
    ///
    /// "CPU"
    #[serde(rename="CPU")]
    CPU,
    

    /// Wallclock time sampling. More expensive as stops all threads.
    ///
    /// "WALL"
    #[serde(rename="WALL")]
    WALL,
    

    /// In-use heap profile. Represents a snapshot of the allocations that are live at the time of the profiling.
    ///
    /// "HEAP"
    #[serde(rename="HEAP")]
    HEAP,
    

    /// Single-shot collection of all thread stacks.
    ///
    /// "THREADS"
    #[serde(rename="THREADS")]
    THREADS,
    

    /// Synchronization contention profile.
    ///
    /// "CONTENTION"
    #[serde(rename="CONTENTION")]
    CONTENTION,
    

    /// Peak heap profile.
    ///
    /// "PEAK_HEAP"
    #[serde(rename="PEAK_HEAP")]
    PEAKHEAP,
    

    /// Heap allocation profile. It represents the aggregation of all allocations made over the duration of the profile. All allocations are included, including those that might have been freed by the end of the profiling interval. The profile is in particular useful for garbage collecting languages to understand which parts of the code create most of the garbage collection pressure to see if those can be optimized.
    ///
    /// "HEAP_ALLOC"
    #[serde(rename="HEAP_ALLOC")]
    HEAPALLOC,
}

impl AsRef<str> for CreateProfileRequestProfileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateProfileRequestProfileTypeEnum::PROFILETYPEUNSPECIFIED => "PROFILE_TYPE_UNSPECIFIED",
            CreateProfileRequestProfileTypeEnum::CPU => "CPU",
            CreateProfileRequestProfileTypeEnum::WALL => "WALL",
            CreateProfileRequestProfileTypeEnum::HEAP => "HEAP",
            CreateProfileRequestProfileTypeEnum::THREADS => "THREADS",
            CreateProfileRequestProfileTypeEnum::CONTENTION => "CONTENTION",
            CreateProfileRequestProfileTypeEnum::PEAKHEAP => "PEAK_HEAP",
            CreateProfileRequestProfileTypeEnum::HEAPALLOC => "HEAP_ALLOC",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateProfileRequestProfileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_TYPE_UNSPECIFIED" => Ok(CreateProfileRequestProfileTypeEnum::PROFILETYPEUNSPECIFIED),
           "CPU" => Ok(CreateProfileRequestProfileTypeEnum::CPU),
           "WALL" => Ok(CreateProfileRequestProfileTypeEnum::WALL),
           "HEAP" => Ok(CreateProfileRequestProfileTypeEnum::HEAP),
           "THREADS" => Ok(CreateProfileRequestProfileTypeEnum::THREADS),
           "CONTENTION" => Ok(CreateProfileRequestProfileTypeEnum::CONTENTION),
           "PEAK_HEAP" => Ok(CreateProfileRequestProfileTypeEnum::PEAKHEAP),
           "HEAP_ALLOC" => Ok(CreateProfileRequestProfileTypeEnum::HEAPALLOC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateProfileRequestProfileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProfileProfileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of profile. For offline mode, this must be specified when creating the profile. For online mode it is assigned and returned by the server.
pub enum ProfileProfileTypeEnum {
    

    /// Unspecified profile type.
    ///
    /// "PROFILE_TYPE_UNSPECIFIED"
    #[serde(rename="PROFILE_TYPE_UNSPECIFIED")]
    PROFILETYPEUNSPECIFIED,
    

    /// Thread CPU time sampling.
    ///
    /// "CPU"
    #[serde(rename="CPU")]
    CPU,
    

    /// Wallclock time sampling. More expensive as stops all threads.
    ///
    /// "WALL"
    #[serde(rename="WALL")]
    WALL,
    

    /// In-use heap profile. Represents a snapshot of the allocations that are live at the time of the profiling.
    ///
    /// "HEAP"
    #[serde(rename="HEAP")]
    HEAP,
    

    /// Single-shot collection of all thread stacks.
    ///
    /// "THREADS"
    #[serde(rename="THREADS")]
    THREADS,
    

    /// Synchronization contention profile.
    ///
    /// "CONTENTION"
    #[serde(rename="CONTENTION")]
    CONTENTION,
    

    /// Peak heap profile.
    ///
    /// "PEAK_HEAP"
    #[serde(rename="PEAK_HEAP")]
    PEAKHEAP,
    

    /// Heap allocation profile. It represents the aggregation of all allocations made over the duration of the profile. All allocations are included, including those that might have been freed by the end of the profiling interval. The profile is in particular useful for garbage collecting languages to understand which parts of the code create most of the garbage collection pressure to see if those can be optimized.
    ///
    /// "HEAP_ALLOC"
    #[serde(rename="HEAP_ALLOC")]
    HEAPALLOC,
}

impl AsRef<str> for ProfileProfileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProfileProfileTypeEnum::PROFILETYPEUNSPECIFIED => "PROFILE_TYPE_UNSPECIFIED",
            ProfileProfileTypeEnum::CPU => "CPU",
            ProfileProfileTypeEnum::WALL => "WALL",
            ProfileProfileTypeEnum::HEAP => "HEAP",
            ProfileProfileTypeEnum::THREADS => "THREADS",
            ProfileProfileTypeEnum::CONTENTION => "CONTENTION",
            ProfileProfileTypeEnum::PEAKHEAP => "PEAK_HEAP",
            ProfileProfileTypeEnum::HEAPALLOC => "HEAP_ALLOC",
        }
    }
}

impl std::convert::TryFrom< &str> for ProfileProfileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_TYPE_UNSPECIFIED" => Ok(ProfileProfileTypeEnum::PROFILETYPEUNSPECIFIED),
           "CPU" => Ok(ProfileProfileTypeEnum::CPU),
           "WALL" => Ok(ProfileProfileTypeEnum::WALL),
           "HEAP" => Ok(ProfileProfileTypeEnum::HEAP),
           "THREADS" => Ok(ProfileProfileTypeEnum::THREADS),
           "CONTENTION" => Ok(ProfileProfileTypeEnum::CONTENTION),
           "PEAK_HEAP" => Ok(ProfileProfileTypeEnum::PEAKHEAP),
           "HEAP_ALLOC" => Ok(ProfileProfileTypeEnum::HEAPALLOC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProfileProfileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


