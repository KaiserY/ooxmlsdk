//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ClipboardFormatValues {
  #[sdk(rename = "PictOld")]
  #[default]
  PictureOld,
  #[sdk(rename = "Pict")]
  Picture,
  #[sdk(rename = "Bitmap")]
  Bitmap,
  #[sdk(rename = "PictPrint")]
  PicturePrint,
  #[sdk(rename = "PictScreen")]
  PictureScreen,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ObjectValues {
  #[sdk(rename = "Button")]
  #[default]
  Button,
  #[sdk(rename = "Checkbox")]
  Checkbox,
  #[sdk(rename = "Dialog")]
  Dialog,
  #[sdk(rename = "Drop")]
  Drop,
  #[sdk(rename = "Edit")]
  Edit,
  #[sdk(rename = "GBox")]
  GroupBox,
  #[sdk(rename = "Label")]
  Label,
  #[sdk(rename = "LineA")]
  AuditingLine,
  #[sdk(rename = "List")]
  List,
  #[sdk(rename = "Movie")]
  Movie,
  #[sdk(rename = "Note")]
  Note,
  #[sdk(rename = "Pict")]
  Picture,
  #[sdk(rename = "Radio")]
  Radio,
  #[sdk(rename = "RectA")]
  AuditingRectangle,
  #[sdk(rename = "Scroll")]
  Scroll,
  #[sdk(rename = "Spin")]
  Spin,
  #[sdk(rename = "Shape")]
  Shape,
  #[sdk(rename = "Group")]
  Group,
  #[sdk(rename = "Rect")]
  Rectangle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BooleanEntryWithBlankValues {
  #[sdk(rename = "True")]
  #[default]
  True,
  #[sdk(rename = "t")]
  T,
  #[sdk(rename = "False")]
  False,
  #[sdk(rename = "f")]
  F,
  Empty,
}
/// Attached Object Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xvml:ClientData")]
pub struct ClientData {
  /// Object type
  #[sdk(attr(qname = ":ObjectType"))]
  pub object_type: ObjectValues,
  #[sdk(
        choice(
            text_child(variant = MoveWithCells, qname = "xvml:MoveWithCells"),
            text_child(variant = ResizeWithCells, qname = "xvml:SizeWithCells"),
            text_child(variant = Anchor, qname = "xvml:Anchor"),
            text_child(variant = Locked, qname = "xvml:Locked"),
            text_child(variant = DefaultSize, qname = "xvml:DefaultSize"),
            text_child(variant = PrintObject, qname = "xvml:PrintObject"),
            text_child(variant = Disabled, qname = "xvml:Disabled"),
            text_child(variant = AutoFill, qname = "xvml:AutoFill"),
            text_child(variant = AutoLine, qname = "xvml:AutoLine"),
            text_child(variant = AutoSizePicture, qname = "xvml:AutoPict"),
            text_child(variant = FormulaMacro, qname = "xvml:FmlaMacro"),
            text_child(variant = HorizontalTextAlignment, qname = "xvml:TextHAlign"),
            text_child(variant = VerticalTextAlignment, qname = "xvml:TextVAlign"),
            text_child(variant = LockText, qname = "xvml:LockText"),
            text_child(variant = JustifyLastLine, qname = "xvml:JustLastX"),
            text_child(variant = SecretEdit, qname = "xvml:SecretEdit"),
            text_child(variant = DefaultButton, qname = "xvml:Default"),
            text_child(variant = HelpButton, qname = "xvml:Help"),
            text_child(variant = CancelButton, qname = "xvml:Cancel"),
            text_child(variant = DismissButton, qname = "xvml:Dismiss"),
            text_child(variant = AcceleratorPrimary, qname = "xvml:Accel"),
            text_child(variant = AcceleratorSecondary, qname = "xvml:Accel2"),
            text_child(variant = CommentRowTarget, qname = "xvml:Row"),
            text_child(variant = CommentColumnTarget, qname = "xvml:Column"),
            text_child(variant = Visible, qname = "xvml:Visible"),
            text_child(variant = RowHidden, qname = "xvml:RowHidden"),
            text_child(variant = ColumnHidden, qname = "xvml:ColHidden"),
            text_child(variant = InputValidationType, qname = "xvml:VTEdit"),
            text_child(variant = MultiLine, qname = "xvml:MultiLine"),
            text_child(variant = VerticalScrollBar, qname = "xvml:VScroll"),
            text_child(variant = ValidIds, qname = "xvml:ValidIds"),
            text_child(variant = FormulaRange, qname = "xvml:FmlaRange"),
            text_child(variant = MinDropDownWidth, qname = "xvml:WidthMin"),
            text_child(variant = SelectionEntry, qname = "xvml:Sel"),
            text_child(
                variant = Disable3DForListBoxAndDropDown,
                qname = "xvml:NoThreeD2"
            ),
            text_child(variant = SelectionType, qname = "xvml:SelType"),
            text_child(variant = MultiSelections, qname = "xvml:MultiSel"),
            text_child(variant = ListBoxCallbackType, qname = "xvml:LCT"),
            text_child(variant = ListItem, qname = "xvml:ListItem"),
            text_child(variant = DropStyle, qname = "xvml:DropStyle"),
            text_child(variant = Colored, qname = "xvml:Colored"),
            text_child(variant = DropLines, qname = "xvml:DropLines"),
            text_child(variant = Checked, qname = "xvml:Checked"),
            text_child(variant = FormulaLink, qname = "xvml:FmlaLink"),
            text_child(variant = FormulaPicture, qname = "xvml:FmlaPict"),
            text_child(variant = Disable3D, qname = "xvml:NoThreeD"),
            text_child(variant = FirstButton, qname = "xvml:FirstButton"),
            text_child(variant = FormulaGroup, qname = "xvml:FmlaGroup"),
            text_child(variant = ScrollBarPosition, qname = "xvml:Val"),
            text_child(variant = ScrollBarMin, qname = "xvml:Min"),
            text_child(variant = ScrollBarMax, qname = "xvml:Max"),
            text_child(variant = ScrollBarIncrement, qname = "xvml:Inc"),
            text_child(variant = ScrollBarPageIncrement, qname = "xvml:Page"),
            text_child(variant = HorizontalScrollBar, qname = "xvml:Horiz"),
            text_child(variant = ScrollBarWidth, qname = "xvml:Dx"),
            text_child(variant = MapOcxControl, qname = "xvml:MapOCX"),
            text_child(variant = ClipboardFormat, qname = "xvml:CF"),
            text_child(variant = CameraObject, qname = "xvml:Camera"),
            text_child(variant = RecalculateAlways, qname = "xvml:RecalcAlways"),
            text_child(variant = AutoScaleFont, qname = "xvml:AutoScale"),
            text_child(variant = DdeObject, qname = "xvml:DDE"),
            text_child(variant = UiObject, qname = "xvml:UIObj"),
            text_child(variant = ScriptText, qname = "xvml:ScriptText"),
            text_child(variant = ScriptExtended, qname = "xvml:ScriptExtended"),
            text_child(variant = ScriptLanguage, qname = "xvml:ScriptLanguage"),
            text_child(variant = ScriptLocation, qname = "xvml:ScriptLocation"),
            text_child(variant = FormulaTextBox, qname = "xvml:FmlaTxbx")
        )
    )]
  pub client_data_choice: Vec<ClientDataChoice>,
}
/// Move with Cells.
pub type MoveWithCells = BooleanEntryWithBlankValues;
/// Resize with Cells.
pub type ResizeWithCells = BooleanEntryWithBlankValues;
/// Lock Toggle.
pub type Locked = BooleanEntryWithBlankValues;
/// Default Size Toggle.
pub type DefaultSize = BooleanEntryWithBlankValues;
/// Print Toggle.
pub type PrintObject = BooleanEntryWithBlankValues;
/// Macro Disable Toggle.
pub type Disabled = BooleanEntryWithBlankValues;
/// AutoFill.
pub type AutoFill = BooleanEntryWithBlankValues;
/// AutoLine.
pub type AutoLine = BooleanEntryWithBlankValues;
/// Automatically Size.
pub type AutoSizePicture = BooleanEntryWithBlankValues;
/// Text Lock.
pub type LockText = BooleanEntryWithBlankValues;
/// East Asia Alignment Toggle.
pub type JustifyLastLine = BooleanEntryWithBlankValues;
/// Password Edit.
pub type SecretEdit = BooleanEntryWithBlankValues;
/// Default Button.
pub type DefaultButton = BooleanEntryWithBlankValues;
/// Help Button.
pub type HelpButton = BooleanEntryWithBlankValues;
/// Cancel Button.
pub type CancelButton = BooleanEntryWithBlankValues;
/// Dismiss Button.
pub type DismissButton = BooleanEntryWithBlankValues;
/// Comment Visibility Toggle.
pub type Visible = BooleanEntryWithBlankValues;
/// Comment's Row is Hidden.
pub type RowHidden = BooleanEntryWithBlankValues;
/// Comment's Column is Hidden.
pub type ColumnHidden = BooleanEntryWithBlankValues;
/// Multi-line.
pub type MultiLine = BooleanEntryWithBlankValues;
/// Vertical Scroll.
pub type VerticalScrollBar = BooleanEntryWithBlankValues;
/// Valid ID.
pub type ValidIds = BooleanEntryWithBlankValues;
/// Disable 3D.
pub type Disable3DForListBoxAndDropDown = BooleanEntryWithBlankValues;
/// Dropdown Color Toggle.
pub type Colored = BooleanEntryWithBlankValues;
/// Disable 3D.
pub type Disable3D = BooleanEntryWithBlankValues;
/// First Radio Button.
pub type FirstButton = BooleanEntryWithBlankValues;
/// Scroll Bar Orientation.
pub type HorizontalScrollBar = BooleanEntryWithBlankValues;
/// ActiveX Control.
pub type MapOcxControl = BooleanEntryWithBlankValues;
/// Camera Tool.
pub type CameraObject = BooleanEntryWithBlankValues;
/// Recalculation Toggle.
pub type RecalculateAlways = BooleanEntryWithBlankValues;
/// Font AutoScale.
pub type AutoScaleFont = BooleanEntryWithBlankValues;
/// Dynamic Data Exchange.
pub type DdeObject = BooleanEntryWithBlankValues;
/// UI Object Toggle.
pub type UiObject = BooleanEntryWithBlankValues;
/// Anchor.
pub type Anchor = crate::simple_type::StringValue;
/// Horizontal Text Alignment.
pub type HorizontalTextAlignment = crate::simple_type::StringValue;
/// Vertical Text Alignment.
pub type VerticalTextAlignment = crate::simple_type::StringValue;
/// List Items Source Range.
pub type FormulaRange = crate::simple_type::StringValue;
/// Selection Type.
pub type SelectionType = crate::simple_type::StringValue;
/// Multiple Selections.
pub type MultiSelections = crate::simple_type::StringValue;
/// Callback Type.
pub type ListBoxCallbackType = crate::simple_type::StringValue;
/// Non-linked List Item.
pub type ListItem = crate::simple_type::StringValue;
/// Dropdown Style.
pub type DropStyle = crate::simple_type::StringValue;
/// Linked Formula.
pub type FormulaLink = crate::simple_type::StringValue;
/// Camera Source Range.
pub type FormulaPicture = crate::simple_type::StringValue;
/// Linked Formula - Group Box.
pub type FormulaGroup = crate::simple_type::StringValue;
/// HTML Script Text.
pub type ScriptText = crate::simple_type::StringValue;
/// HTML Script Attributes.
pub type ScriptExtended = crate::simple_type::StringValue;
/// Text Formula.
pub type FormulaTextBox = crate::simple_type::StringValue;
/// Reference to Custom Function.
pub type FormulaMacro = crate::simple_type::StringValue;
/// Primary Keyboard Accelerator.
pub type AcceleratorPrimary = crate::simple_type::ByteValue;
/// Secondary Keyboard Accelerator.
pub type AcceleratorSecondary = crate::simple_type::ByteValue;
/// Comment Row Target.
pub type CommentRowTarget = crate::simple_type::IntegerValue;
/// Comment Column Target.
pub type CommentColumnTarget = crate::simple_type::IntegerValue;
/// Validation Type.
pub type InputValidationType = crate::simple_type::IntegerValue;
/// Minimum Width.
pub type MinDropDownWidth = crate::simple_type::IntegerValue;
/// Selected Entry.
pub type SelectionEntry = crate::simple_type::IntegerValue;
/// Dropdown Maximum Lines.
pub type DropLines = crate::simple_type::IntegerValue;
/// Checked.
pub type Checked = crate::simple_type::IntegerValue;
/// Scroll bar position.
pub type ScrollBarPosition = crate::simple_type::IntegerValue;
/// Scroll Bar Minimum.
pub type ScrollBarMin = crate::simple_type::IntegerValue;
/// Scroll Bar Maximum.
pub type ScrollBarMax = crate::simple_type::IntegerValue;
/// Scroll Bar Increment.
pub type ScrollBarIncrement = crate::simple_type::IntegerValue;
/// Scroll Bar Page Increment.
pub type ScrollBarPageIncrement = crate::simple_type::IntegerValue;
/// Scroll Bar Width.
pub type ScrollBarWidth = crate::simple_type::IntegerValue;
/// Clipboard Format.
pub type ClipboardFormat = ClipboardFormatValues;
/// HTML Script Language.
pub type ScriptLanguage = crate::simple_type::IntegerValue;
/// HTML Script Location.
pub type ScriptLocation = crate::simple_type::IntegerValue;
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ClientDataChoice {
  /// Move with Cells.
  #[sdk(text_child(qname = "xvml:MoveWithCells"))]
  MoveWithCells(MoveWithCells),
  /// Resize with Cells.
  #[sdk(text_child(qname = "xvml:SizeWithCells"))]
  ResizeWithCells(ResizeWithCells),
  /// Anchor.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:Anchor"))]
  Anchor(Anchor),
  /// Lock Toggle.
  #[sdk(text_child(qname = "xvml:Locked"))]
  Locked(Locked),
  /// Default Size Toggle.
  #[sdk(text_child(qname = "xvml:DefaultSize"))]
  DefaultSize(DefaultSize),
  /// Print Toggle.
  #[sdk(text_child(qname = "xvml:PrintObject"))]
  PrintObject(PrintObject),
  /// Macro Disable Toggle.
  #[sdk(text_child(qname = "xvml:Disabled"))]
  Disabled(Disabled),
  /// AutoFill.
  #[sdk(text_child(qname = "xvml:AutoFill"))]
  AutoFill(AutoFill),
  /// AutoLine.
  #[sdk(text_child(qname = "xvml:AutoLine"))]
  AutoLine(AutoLine),
  /// Automatically Size.
  #[sdk(text_child(qname = "xvml:AutoPict"))]
  AutoSizePicture(AutoSizePicture),
  /// Reference to Custom Function.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:FmlaMacro"))]
  FormulaMacro(FormulaMacro),
  /// Horizontal Text Alignment.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:TextHAlign"))]
  HorizontalTextAlignment(HorizontalTextAlignment),
  /// Vertical Text Alignment.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:TextVAlign"))]
  VerticalTextAlignment(VerticalTextAlignment),
  /// Text Lock.
  #[sdk(text_child(qname = "xvml:LockText"))]
  LockText(LockText),
  /// East Asia Alignment Toggle.
  #[sdk(text_child(qname = "xvml:JustLastX"))]
  JustifyLastLine(JustifyLastLine),
  /// Password Edit.
  #[sdk(text_child(qname = "xvml:SecretEdit"))]
  SecretEdit(SecretEdit),
  /// Default Button.
  #[sdk(text_child(qname = "xvml:Default"))]
  DefaultButton(DefaultButton),
  /// Help Button.
  #[sdk(text_child(qname = "xvml:Help"))]
  HelpButton(HelpButton),
  /// Cancel Button.
  #[sdk(text_child(qname = "xvml:Cancel"))]
  CancelButton(CancelButton),
  /// Dismiss Button.
  #[sdk(text_child(qname = "xvml:Dismiss"))]
  DismissButton(DismissButton),
  /// Primary Keyboard Accelerator.
  #[sdk(text_child(simple_type = "ByteValue", qname = "xvml:Accel"))]
  AcceleratorPrimary(AcceleratorPrimary),
  /// Secondary Keyboard Accelerator.
  #[sdk(text_child(simple_type = "ByteValue", qname = "xvml:Accel2"))]
  AcceleratorSecondary(AcceleratorSecondary),
  /// Comment Row Target.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Row"))]
  CommentRowTarget(CommentRowTarget),
  /// Comment Column Target.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Column"))]
  CommentColumnTarget(CommentColumnTarget),
  /// Comment Visibility Toggle.
  #[sdk(text_child(qname = "xvml:Visible"))]
  Visible(Visible),
  /// Comment's Row is Hidden.
  #[sdk(text_child(qname = "xvml:RowHidden"))]
  RowHidden(RowHidden),
  /// Comment's Column is Hidden.
  #[sdk(text_child(qname = "xvml:ColHidden"))]
  ColumnHidden(ColumnHidden),
  /// Validation Type.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:VTEdit"))]
  InputValidationType(InputValidationType),
  /// Multi-line.
  #[sdk(text_child(qname = "xvml:MultiLine"))]
  MultiLine(MultiLine),
  /// Vertical Scroll.
  #[sdk(text_child(qname = "xvml:VScroll"))]
  VerticalScrollBar(VerticalScrollBar),
  /// Valid ID.
  #[sdk(text_child(qname = "xvml:ValidIds"))]
  ValidIds(ValidIds),
  /// List Items Source Range.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:FmlaRange"))]
  FormulaRange(FormulaRange),
  /// Minimum Width.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:WidthMin"))]
  MinDropDownWidth(MinDropDownWidth),
  /// Selected Entry.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Sel"))]
  SelectionEntry(SelectionEntry),
  /// Disable 3D.
  #[sdk(text_child(qname = "xvml:NoThreeD2"))]
  Disable3DForListBoxAndDropDown(Disable3DForListBoxAndDropDown),
  /// Selection Type.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:SelType"))]
  SelectionType(SelectionType),
  /// Multiple Selections.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:MultiSel"))]
  MultiSelections(MultiSelections),
  /// Callback Type.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:LCT"))]
  ListBoxCallbackType(ListBoxCallbackType),
  /// Non-linked List Item.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:ListItem"))]
  ListItem(ListItem),
  /// Dropdown Style.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:DropStyle"))]
  DropStyle(DropStyle),
  /// Dropdown Color Toggle.
  #[sdk(text_child(qname = "xvml:Colored"))]
  Colored(Colored),
  /// Dropdown Maximum Lines.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:DropLines"))]
  DropLines(DropLines),
  /// Checked.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Checked"))]
  Checked(Checked),
  /// Linked Formula.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:FmlaLink"))]
  FormulaLink(FormulaLink),
  /// Camera Source Range.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:FmlaPict"))]
  FormulaPicture(FormulaPicture),
  /// Disable 3D.
  #[sdk(text_child(qname = "xvml:NoThreeD"))]
  Disable3D(Disable3D),
  /// First Radio Button.
  #[sdk(text_child(qname = "xvml:FirstButton"))]
  FirstButton(FirstButton),
  /// Linked Formula - Group Box.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:FmlaGroup"))]
  FormulaGroup(FormulaGroup),
  /// Scroll bar position.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Val"))]
  ScrollBarPosition(ScrollBarPosition),
  /// Scroll Bar Minimum.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Min"))]
  ScrollBarMin(ScrollBarMin),
  /// Scroll Bar Maximum.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Max"))]
  ScrollBarMax(ScrollBarMax),
  /// Scroll Bar Increment.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Inc"))]
  ScrollBarIncrement(ScrollBarIncrement),
  /// Scroll Bar Page Increment.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Page"))]
  ScrollBarPageIncrement(ScrollBarPageIncrement),
  /// Scroll Bar Orientation.
  #[sdk(text_child(qname = "xvml:Horiz"))]
  HorizontalScrollBar(HorizontalScrollBar),
  /// Scroll Bar Width.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:Dx"))]
  ScrollBarWidth(ScrollBarWidth),
  /// ActiveX Control.
  #[sdk(text_child(qname = "xvml:MapOCX"))]
  MapOcxControl(MapOcxControl),
  /// Clipboard Format.
  #[sdk(text_child(qname = "xvml:CF"))]
  ClipboardFormat(ClipboardFormat),
  /// Camera Tool.
  #[sdk(text_child(qname = "xvml:Camera"))]
  CameraObject(CameraObject),
  /// Recalculation Toggle.
  #[sdk(text_child(qname = "xvml:RecalcAlways"))]
  RecalculateAlways(RecalculateAlways),
  /// Font AutoScale.
  #[sdk(text_child(qname = "xvml:AutoScale"))]
  AutoScaleFont(AutoScaleFont),
  /// Dynamic Data Exchange.
  #[sdk(text_child(qname = "xvml:DDE"))]
  DdeObject(DdeObject),
  /// UI Object Toggle.
  #[sdk(text_child(qname = "xvml:UIObj"))]
  UiObject(UiObject),
  /// HTML Script Text.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:ScriptText"))]
  ScriptText(ScriptText),
  /// HTML Script Attributes.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:ScriptExtended"))]
  ScriptExtended(ScriptExtended),
  /// HTML Script Language.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:ScriptLanguage"))]
  ScriptLanguage(ScriptLanguage),
  /// HTML Script Location.
  #[sdk(text_child(simple_type = "IntegerValue", qname = "xvml:ScriptLocation"))]
  ScriptLocation(ScriptLocation),
  /// Text Formula.
  #[sdk(text_child(simple_type = "StringValue", qname = "xvml:FmlaTxbx"))]
  FormulaTextBox(FormulaTextBox),
}
