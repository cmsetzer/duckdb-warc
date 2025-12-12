use duckdb::core::LogicalTypeId;
use duckdb::core::LogicalTypeId::{Integer, Timestamp, Varchar};

use warc::WarcHeader;
use warc::WarcHeader::{
    BlockDigest, ConcurrentTo, ContentLength, ContentType, Date, Filename, IPAddress,
    IdentifiedPayloadType, PayloadDigest, Profile, RecordID, RefersTo, SegmentNumber,
    SegmentOriginID, SegmentTotalLength, TargetURI, Truncated, WarcInfoID, WarcType,
};

#[derive(Debug)]
pub struct Field {
    pub header: WarcHeader,
    pub field_type: LogicalTypeId,
}

pub static WARC_FIELDS: &[Field] = &[
    Field { header: WarcType, field_type: Varchar },
    Field { header: TargetURI, field_type: Varchar },
    Field { header: Date, field_type: Timestamp },
    Field { header: Profile, field_type: Varchar },
    Field { header: RecordID, field_type: Varchar },
    Field { header: RefersTo, field_type: Varchar },
    Field { header: ContentType, field_type: Varchar },
    Field { header: ContentLength, field_type: Integer },
    Field { header: BlockDigest, field_type: Varchar },
    Field { header: ConcurrentTo, field_type: Varchar },
    Field { header: Filename, field_type: Varchar },
    Field { header: IdentifiedPayloadType, field_type: Varchar },
    Field { header: IPAddress, field_type: Varchar },
    Field { header: PayloadDigest, field_type: Varchar },
    Field { header: SegmentNumber, field_type: Integer },
    Field { header: SegmentOriginID, field_type: Varchar },
    Field { header: SegmentTotalLength, field_type: Integer },
    Field { header: Truncated, field_type: Varchar },
    Field { header: WarcInfoID, field_type: Varchar },
];
