//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataSourceValues {
  #[sdk(rename = "ArticleInAPeriodical")]
  #[default]
  ArticleInAPeriodical,
  #[sdk(rename = "Book")]
  Book,
  #[sdk(rename = "BookSection")]
  BookSection,
  #[sdk(rename = "JournalArticle")]
  JournalArticle,
  #[sdk(rename = "ConferenceProceedings")]
  ConferenceProceedings,
  #[sdk(rename = "Report")]
  Report,
  #[sdk(rename = "SoundRecording")]
  SoundRecording,
  #[sdk(rename = "Performance")]
  Performance,
  #[sdk(rename = "Art")]
  Art,
  #[sdk(rename = "DocumentFromInternetSite")]
  DocumentFromInternetSite,
  #[sdk(rename = "InternetSite")]
  InternetSite,
  #[sdk(rename = "Film")]
  Film,
  #[sdk(rename = "Interview")]
  Interview,
  #[sdk(rename = "Patent")]
  Patent,
  #[sdk(rename = "ElectronicSource")]
  ElectronicSource,
  #[sdk(rename = "Case")]
  Case,
  #[sdk(rename = "Misc")]
  Miscellaneous,
}
/// Sources.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Sources")]
pub struct Sources {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Selected Style
  #[sdk(attr(qname = ":SelectedStyle"))]
  #[sdk(string_length(min = 0u32, max = 255u32))]
  pub selected_style: Option<crate::simple_type::StringValue>,
  /// Documentation Style Name
  #[sdk(attr(qname = ":StyleName"))]
  #[sdk(string_length(min = 0u32, max = 255u32))]
  pub style_name: Option<crate::simple_type::StringValue>,
  /// Uniform Resource Identifier
  #[sdk(attr(qname = ":URI"))]
  #[sdk(string_length(min = 0u32, max = 255u32))]
  pub uri: Option<crate::simple_type::StringValue>,
  /// Source.
  #[sdk(child(qname = "b:Source"))]
  pub source: Vec<Source>,
}
/// Person.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Person")]
pub struct Person {
  /// Person's Last, or Family, Name.
  #[sdk(text_child(qname = "b:Last"))]
  pub last: Vec<Last>,
  /// Person's First, or Given, Name.
  #[sdk(text_child(qname = "b:First"))]
  pub first: Vec<First>,
  /// Person's Middle, or Other, Name.
  #[sdk(text_child(qname = "b:Middle"))]
  pub middle: Vec<Middle>,
}
/// Person's Last, or Family, Name.
pub type Last = crate::simple_type::StringValue;
/// Person's First, or Given, Name.
pub type First = crate::simple_type::StringValue;
/// Person's Middle, or Other, Name.
pub type Middle = crate::simple_type::StringValue;
/// Corporate Author.
pub type Corporate = crate::simple_type::StringValue;
/// Abbreviated Case Number.
pub type AbbreviatedCaseNumber = crate::simple_type::StringValue;
/// Album Title.
pub type AlbumTitle = crate::simple_type::StringValue;
/// Book Title.
pub type BookTitle = crate::simple_type::StringValue;
/// Broadcaster.
pub type Broadcaster = crate::simple_type::StringValue;
/// Broadcast Title.
pub type BroadcastTitle = crate::simple_type::StringValue;
/// Case Number.
pub type CaseNumber = crate::simple_type::StringValue;
/// Chapter Number.
pub type ChapterNumber = crate::simple_type::StringValue;
/// City.
pub type City = crate::simple_type::StringValue;
/// Comments.
pub type Comments = crate::simple_type::StringValue;
/// Conference or Proceedings Name.
pub type ConferenceName = crate::simple_type::StringValue;
/// Country or Region.
pub type CountryRegion = crate::simple_type::StringValue;
/// Court.
pub type Court = crate::simple_type::StringValue;
/// Day.
pub type Day = crate::simple_type::StringValue;
/// Day Accessed.
pub type DayAccessed = crate::simple_type::StringValue;
/// Department.
pub type Department = crate::simple_type::StringValue;
/// Distributor.
pub type Distributor = crate::simple_type::StringValue;
/// Editor.
pub type Edition = crate::simple_type::StringValue;
/// GUID.
pub type GuidString = crate::simple_type::StringValue;
/// Institution.
pub type Institution = crate::simple_type::StringValue;
/// Internet Site Title.
pub type InternetSiteTitle = crate::simple_type::StringValue;
/// Issue.
pub type Issue = crate::simple_type::StringValue;
/// Journal Name.
pub type JournalName = crate::simple_type::StringValue;
/// Locale ID.
pub type LcId = crate::simple_type::StringValue;
/// Medium.
pub type Medium = crate::simple_type::StringValue;
/// Month.
pub type Month = crate::simple_type::StringValue;
/// Month Accessed.
pub type MonthAccessed = crate::simple_type::StringValue;
/// Number of Volumes.
pub type NumberVolumes = crate::simple_type::StringValue;
/// Pages.
pub type Pages = crate::simple_type::StringValue;
/// Patent Number.
pub type PatentNumber = crate::simple_type::StringValue;
/// Periodical Title.
pub type PeriodicalTitle = crate::simple_type::StringValue;
/// Production Company.
pub type ProductionCompany = crate::simple_type::StringValue;
/// Publication Title.
pub type PublicationTitle = crate::simple_type::StringValue;
/// Publisher.
pub type Publisher = crate::simple_type::StringValue;
/// Recording Number.
pub type RecordingNumber = crate::simple_type::StringValue;
/// Reference Order.
pub type ReferenceOrder = crate::simple_type::StringValue;
/// Reporter.
pub type Reporter = crate::simple_type::StringValue;
/// Short Title.
pub type ShortTitle = crate::simple_type::StringValue;
/// Standard Number.
pub type StandardNumber = crate::simple_type::StringValue;
/// State or Province.
pub type StateProvince = crate::simple_type::StringValue;
/// Station.
pub type Station = crate::simple_type::StringValue;
/// Tag.
pub type Tag = crate::simple_type::StringValue;
/// Theater.
pub type Theater = crate::simple_type::StringValue;
/// Thesis Type.
pub type ThesisType = crate::simple_type::StringValue;
/// Title.
pub type Title = crate::simple_type::StringValue;
/// Type.
pub type PatentType = crate::simple_type::StringValue;
/// URL.
pub type UrlString = crate::simple_type::StringValue;
/// Version.
pub type Version = crate::simple_type::StringValue;
/// Volume.
pub type Volume = crate::simple_type::StringValue;
/// Year.
pub type Year = crate::simple_type::StringValue;
/// Year Accessed.
pub type YearAccessed = crate::simple_type::StringValue;
/// Name List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:NameList")]
pub struct NameList {
  /// Person.
  #[sdk(child(qname = "b:Person"))]
  pub person: Vec<Person>,
}
/// Artist.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Artist")]
pub struct Artist {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Book Author.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:BookAuthor")]
pub struct BookAuthor {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Compiler.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Compiler")]
pub struct Compiler {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Composer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Composer")]
pub struct Composer {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Conductor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Conductor")]
pub struct Conductor {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Counsel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Counsel")]
pub struct Counsel {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Director.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Director")]
pub struct Director {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Editor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Editor")]
pub struct Editor {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Interviewee.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Interviewee")]
pub struct Interviewee {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Interviewer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Interviewer")]
pub struct Interviewer {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Inventor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Inventor")]
pub struct Inventor {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Producer Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:ProducerName")]
pub struct ProducerName {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Translator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Translator")]
pub struct Translator {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Writer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Writer")]
pub struct Writer {
  /// Name List
  #[sdk(child(qname = "b:NameList"))]
  pub name_list: std::boxed::Box<NameList>,
}
/// Author.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Author")]
pub struct Author {
  #[sdk(
        choice(
            child(variant = NameList, qname = "b:NameList"),
            text_child(variant = Corporate, qname = "b:Corporate")
        )
    )]
  pub author_choice: Option<AuthorChoice>,
}
/// Performer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Performer")]
pub struct Performer {
  #[sdk(
        choice(
            child(variant = NameList, qname = "b:NameList"),
            text_child(variant = Corporate, qname = "b:Corporate")
        )
    )]
  pub performer_choice: Option<PerformerChoice>,
}
/// Contributors List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Author")]
pub struct AuthorList {
  #[sdk(
        choice(
            child(variant = Artist, qname = "b:Artist"),
            child(variant = Author, qname = "b:Author"),
            child(variant = BookAuthor, qname = "b:BookAuthor"),
            child(variant = Compiler, qname = "b:Compiler"),
            child(variant = Composer, qname = "b:Composer"),
            child(variant = Conductor, qname = "b:Conductor"),
            child(variant = Counsel, qname = "b:Counsel"),
            child(variant = Director, qname = "b:Director"),
            child(variant = Editor, qname = "b:Editor"),
            child(variant = Interviewee, qname = "b:Interviewee"),
            child(variant = Interviewer, qname = "b:Interviewer"),
            child(variant = Inventor, qname = "b:Inventor"),
            child(variant = Performer, qname = "b:Performer"),
            child(variant = ProducerName, qname = "b:ProducerName"),
            child(variant = Translator, qname = "b:Translator"),
            child(variant = Writer, qname = "b:Writer")
        )
    )]
  pub author_list_choice: Vec<AuthorListChoice>,
}
/// Source Type.
pub type SourceType = DataSourceValues;
/// Source.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "b:Source")]
pub struct Source {
  #[sdk(
        choice(
            text_child(
                variant = AbbreviatedCaseNumber,
                qname = "b:AbbreviatedCaseNumber"
            ),
            text_child(variant = AlbumTitle, qname = "b:AlbumTitle"),
            child(variant = AuthorList, qname = "b:Author"),
            text_child(variant = BookTitle, qname = "b:BookTitle"),
            text_child(variant = Broadcaster, qname = "b:Broadcaster"),
            text_child(variant = BroadcastTitle, qname = "b:BroadcastTitle"),
            text_child(variant = CaseNumber, qname = "b:CaseNumber"),
            text_child(variant = ChapterNumber, qname = "b:ChapterNumber"),
            text_child(variant = City, qname = "b:City"),
            text_child(variant = Comments, qname = "b:Comments"),
            text_child(variant = ConferenceName, qname = "b:ConferenceName"),
            text_child(variant = CountryRegion, qname = "b:CountryRegion"),
            text_child(variant = Court, qname = "b:Court"),
            text_child(variant = Day, qname = "b:Day"),
            text_child(variant = DayAccessed, qname = "b:DayAccessed"),
            text_child(variant = Department, qname = "b:Department"),
            text_child(variant = Distributor, qname = "b:Distributor"),
            text_child(variant = Edition, qname = "b:Edition"),
            text_child(variant = GuidString, qname = "b:Guid"),
            text_child(variant = Institution, qname = "b:Institution"),
            text_child(variant = InternetSiteTitle, qname = "b:InternetSiteTitle"),
            text_child(variant = Issue, qname = "b:Issue"),
            text_child(variant = JournalName, qname = "b:JournalName"),
            text_child(variant = LcId, qname = "b:LCID"),
            text_child(variant = Medium, qname = "b:Medium"),
            text_child(variant = Month, qname = "b:Month"),
            text_child(variant = MonthAccessed, qname = "b:MonthAccessed"),
            text_child(variant = NumberVolumes, qname = "b:NumberVolumes"),
            text_child(variant = Pages, qname = "b:Pages"),
            text_child(variant = PatentNumber, qname = "b:PatentNumber"),
            text_child(variant = PeriodicalTitle, qname = "b:PeriodicalTitle"),
            text_child(variant = ProductionCompany, qname = "b:ProductionCompany"),
            text_child(variant = PublicationTitle, qname = "b:PublicationTitle"),
            text_child(variant = Publisher, qname = "b:Publisher"),
            text_child(variant = RecordingNumber, qname = "b:RecordingNumber"),
            text_child(variant = ReferenceOrder, qname = "b:RefOrder"),
            text_child(variant = Reporter, qname = "b:Reporter"),
            text_child(variant = SourceType, enum, qname = "b:SourceType"),
            text_child(variant = ShortTitle, qname = "b:ShortTitle"),
            text_child(variant = StandardNumber, qname = "b:StandardNumber"),
            text_child(variant = StateProvince, qname = "b:StateProvince"),
            text_child(variant = Station, qname = "b:Station"),
            text_child(variant = Tag, qname = "b:Tag"),
            text_child(variant = Theater, qname = "b:Theater"),
            text_child(variant = ThesisType, qname = "b:ThesisType"),
            text_child(variant = Title, qname = "b:Title"),
            text_child(variant = PatentType, qname = "b:Type"),
            text_child(variant = UrlString, qname = "b:URL"),
            text_child(variant = Version, qname = "b:Version"),
            text_child(variant = Volume, qname = "b:Volume"),
            text_child(variant = Year, qname = "b:Year"),
            text_child(variant = YearAccessed, qname = "b:YearAccessed")
        )
    )]
  pub source_choice: Vec<SourceChoice>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorChoice {
  /// Name List.
  NameList(std::boxed::Box<NameList>),
  /// Corporate Author.
  Corporate(Corporate),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PerformerChoice {
  /// Name List.
  NameList(std::boxed::Box<NameList>),
  /// Corporate Author.
  Corporate(Corporate),
}
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorListChoice {
  /// Artist.
  Artist(std::boxed::Box<Artist>),
  /// Author.
  Author(std::boxed::Box<Author>),
  /// Book Author.
  BookAuthor(std::boxed::Box<BookAuthor>),
  /// Compiler.
  Compiler(std::boxed::Box<Compiler>),
  /// Composer.
  Composer(std::boxed::Box<Composer>),
  /// Conductor.
  Conductor(std::boxed::Box<Conductor>),
  /// Counsel.
  Counsel(std::boxed::Box<Counsel>),
  /// Director.
  Director(std::boxed::Box<Director>),
  /// Editor.
  Editor(std::boxed::Box<Editor>),
  /// Interviewee.
  Interviewee(std::boxed::Box<Interviewee>),
  /// Interviewer.
  Interviewer(std::boxed::Box<Interviewer>),
  /// Inventor.
  Inventor(std::boxed::Box<Inventor>),
  /// Performer.
  Performer(std::boxed::Box<Performer>),
  /// Producer Name.
  ProducerName(std::boxed::Box<ProducerName>),
  /// Translator.
  Translator(std::boxed::Box<Translator>),
  /// Writer.
  Writer(std::boxed::Box<Writer>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SourceChoice {
  /// Abbreviated Case Number.
  AbbreviatedCaseNumber(AbbreviatedCaseNumber),
  /// Album Title.
  AlbumTitle(AlbumTitle),
  /// Contributors List.
  AuthorList(std::boxed::Box<AuthorList>),
  /// Book Title.
  BookTitle(BookTitle),
  /// Broadcaster.
  Broadcaster(Broadcaster),
  /// Broadcast Title.
  BroadcastTitle(BroadcastTitle),
  /// Case Number.
  CaseNumber(CaseNumber),
  /// Chapter Number.
  ChapterNumber(ChapterNumber),
  /// City.
  City(City),
  /// Comments.
  Comments(Comments),
  /// Conference or Proceedings Name.
  ConferenceName(ConferenceName),
  /// Country or Region.
  CountryRegion(CountryRegion),
  /// Court.
  Court(Court),
  /// Day.
  Day(Day),
  /// Day Accessed.
  DayAccessed(DayAccessed),
  /// Department.
  Department(Department),
  /// Distributor.
  Distributor(Distributor),
  /// Editor.
  Edition(Edition),
  /// GUID.
  GuidString(GuidString),
  /// Institution.
  Institution(Institution),
  /// Internet Site Title.
  InternetSiteTitle(InternetSiteTitle),
  /// Issue.
  Issue(Issue),
  /// Journal Name.
  JournalName(JournalName),
  /// Locale ID.
  LcId(LcId),
  /// Medium.
  Medium(Medium),
  /// Month.
  Month(Month),
  /// Month Accessed.
  MonthAccessed(MonthAccessed),
  /// Number of Volumes.
  NumberVolumes(NumberVolumes),
  /// Pages.
  Pages(Pages),
  /// Patent Number.
  PatentNumber(PatentNumber),
  /// Periodical Title.
  PeriodicalTitle(PeriodicalTitle),
  /// Production Company.
  ProductionCompany(ProductionCompany),
  /// Publication Title.
  PublicationTitle(PublicationTitle),
  /// Publisher.
  Publisher(Publisher),
  /// Recording Number.
  RecordingNumber(RecordingNumber),
  /// Reference Order.
  ReferenceOrder(ReferenceOrder),
  /// Reporter.
  Reporter(Reporter),
  /// Source Type.
  SourceType(SourceType),
  /// Short Title.
  ShortTitle(ShortTitle),
  /// Standard Number.
  StandardNumber(StandardNumber),
  /// State or Province.
  StateProvince(StateProvince),
  /// Station.
  Station(Station),
  /// Tag.
  Tag(Tag),
  /// Theater.
  Theater(Theater),
  /// Thesis Type.
  ThesisType(ThesisType),
  /// Title.
  Title(Title),
  /// Type.
  PatentType(PatentType),
  /// URL.
  UrlString(UrlString),
  /// Version.
  Version(Version),
  /// Volume.
  Volume(Volume),
  /// Year.
  Year(Year),
  /// Year Accessed.
  YearAccessed(YearAccessed),
}
