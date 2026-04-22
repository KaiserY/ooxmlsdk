//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:ClientData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xvml:CT_ClientData/xvml:ClientData")]
pub struct ClientData {
  /// Object type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ObjectType
  #[sdk(attr(qname = ":ObjectType"))]
  pub object_type: ObjectValues,
  #[sdk(choice(
    qname = "xvml:ST_TrueFalseBlank/xvml:MoveWithCells",
    qname = "xvml:ST_TrueFalseBlank/xvml:SizeWithCells",
    qname = "xsd:string/xvml:Anchor",
    qname = "xvml:ST_TrueFalseBlank/xvml:Locked",
    qname = "xvml:ST_TrueFalseBlank/xvml:DefaultSize",
    qname = "xvml:ST_TrueFalseBlank/xvml:PrintObject",
    qname = "xvml:ST_TrueFalseBlank/xvml:Disabled",
    qname = "xvml:ST_TrueFalseBlank/xvml:AutoFill",
    qname = "xvml:ST_TrueFalseBlank/xvml:AutoLine",
    qname = "xvml:ST_TrueFalseBlank/xvml:AutoPict",
    qname = "xvml:ST_Macro/xvml:FmlaMacro",
    qname = "xsd:string/xvml:TextHAlign",
    qname = "xsd:string/xvml:TextVAlign",
    qname = "xvml:ST_TrueFalseBlank/xvml:LockText",
    qname = "xvml:ST_TrueFalseBlank/xvml:JustLastX",
    qname = "xvml:ST_TrueFalseBlank/xvml:SecretEdit",
    qname = "xvml:ST_TrueFalseBlank/xvml:Default",
    qname = "xvml:ST_TrueFalseBlank/xvml:Help",
    qname = "xvml:ST_TrueFalseBlank/xvml:Cancel",
    qname = "xvml:ST_TrueFalseBlank/xvml:Dismiss",
    qname = "xsd:unsignedByte/xvml:Accel",
    qname = "xsd:unsignedByte/xvml:Accel2",
    qname = "xsd:integer/xvml:Row",
    qname = "xsd:integer/xvml:Column",
    qname = "xvml:ST_TrueFalseBlank/xvml:Visible",
    qname = "xvml:ST_TrueFalseBlank/xvml:RowHidden",
    qname = "xvml:ST_TrueFalseBlank/xvml:ColHidden",
    qname = "xsd:integer/xvml:VTEdit",
    qname = "xvml:ST_TrueFalseBlank/xvml:MultiLine",
    qname = "xvml:ST_TrueFalseBlank/xvml:VScroll",
    qname = "xvml:ST_TrueFalseBlank/xvml:ValidIds",
    qname = "xsd:string/xvml:FmlaRange",
    qname = "xsd:integer/xvml:WidthMin",
    qname = "xsd:integer/xvml:Sel",
    qname = "xvml:ST_TrueFalseBlank/xvml:NoThreeD2",
    qname = "xsd:string/xvml:SelType",
    qname = "xsd:string/xvml:MultiSel",
    qname = "xsd:string/xvml:LCT",
    qname = "xsd:string/xvml:ListItem",
    qname = "xsd:string/xvml:DropStyle",
    qname = "xvml:ST_TrueFalseBlank/xvml:Colored",
    qname = "xsd:integer/xvml:DropLines",
    qname = "xsd:integer/xvml:Checked",
    qname = "xsd:string/xvml:FmlaLink",
    qname = "xsd:string/xvml:FmlaPict",
    qname = "xvml:ST_TrueFalseBlank/xvml:NoThreeD",
    qname = "xvml:ST_TrueFalseBlank/xvml:FirstButton",
    qname = "xsd:string/xvml:FmlaGroup",
    qname = "xsd:integer/xvml:Val",
    qname = "xsd:integer/xvml:Min",
    qname = "xsd:integer/xvml:Max",
    qname = "xsd:integer/xvml:Inc",
    qname = "xsd:integer/xvml:Page",
    qname = "xvml:ST_TrueFalseBlank/xvml:Horiz",
    qname = "xsd:integer/xvml:Dx",
    qname = "xvml:ST_TrueFalseBlank/xvml:MapOCX",
    qname = "xvml:ST_CF/xvml:CF",
    qname = "xvml:ST_TrueFalseBlank/xvml:Camera",
    qname = "xvml:ST_TrueFalseBlank/xvml:RecalcAlways",
    qname = "xvml:ST_TrueFalseBlank/xvml:AutoScale",
    qname = "xvml:ST_TrueFalseBlank/xvml:DDE",
    qname = "xvml:ST_TrueFalseBlank/xvml:UIObj",
    qname = "xsd:string/xvml:ScriptText",
    qname = "xsd:string/xvml:ScriptExtended",
    qname = "xsd:nonNegativeInteger/xvml:ScriptLanguage",
    qname = "xsd:nonNegativeInteger/xvml:ScriptLocation",
    qname = "xsd:string/xvml:FmlaTxbx"
  ))]
  pub xml_children: Vec<ClientDataChoice>,
}
/// Move with Cells.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:MoveWithCells.
pub type MoveWithCells = BooleanEntryWithBlankValues;
/// Resize with Cells.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:SizeWithCells.
pub type ResizeWithCells = BooleanEntryWithBlankValues;
/// Lock Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Locked.
pub type Locked = BooleanEntryWithBlankValues;
/// Default Size Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:DefaultSize.
pub type DefaultSize = BooleanEntryWithBlankValues;
/// Print Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:PrintObject.
pub type PrintObject = BooleanEntryWithBlankValues;
/// Macro Disable Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Disabled.
pub type Disabled = BooleanEntryWithBlankValues;
/// AutoFill.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:AutoFill.
pub type AutoFill = BooleanEntryWithBlankValues;
/// AutoLine.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:AutoLine.
pub type AutoLine = BooleanEntryWithBlankValues;
/// Automatically Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:AutoPict.
pub type AutoSizePicture = BooleanEntryWithBlankValues;
/// Text Lock.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:LockText.
pub type LockText = BooleanEntryWithBlankValues;
/// East Asia Alignment Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:JustLastX.
pub type JustifyLastLine = BooleanEntryWithBlankValues;
/// Password Edit.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:SecretEdit.
pub type SecretEdit = BooleanEntryWithBlankValues;
/// Default Button.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Default.
pub type DefaultButton = BooleanEntryWithBlankValues;
/// Help Button.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Help.
pub type HelpButton = BooleanEntryWithBlankValues;
/// Cancel Button.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Cancel.
pub type CancelButton = BooleanEntryWithBlankValues;
/// Dismiss Button.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Dismiss.
pub type DismissButton = BooleanEntryWithBlankValues;
/// Comment Visibility Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Visible.
pub type Visible = BooleanEntryWithBlankValues;
/// Comment's Row is Hidden.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:RowHidden.
pub type RowHidden = BooleanEntryWithBlankValues;
/// Comment's Column is Hidden.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:ColHidden.
pub type ColumnHidden = BooleanEntryWithBlankValues;
/// Multi-line.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:MultiLine.
pub type MultiLine = BooleanEntryWithBlankValues;
/// Vertical Scroll.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:VScroll.
pub type VerticalScrollBar = BooleanEntryWithBlankValues;
/// Valid ID.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:ValidIds.
pub type ValidIds = BooleanEntryWithBlankValues;
/// Disable 3D.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:NoThreeD2.
pub type Disable3DForListBoxAndDropDown = BooleanEntryWithBlankValues;
/// Dropdown Color Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Colored.
pub type Colored = BooleanEntryWithBlankValues;
/// Disable 3D.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:NoThreeD.
pub type Disable3D = BooleanEntryWithBlankValues;
/// First Radio Button.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:FirstButton.
pub type FirstButton = BooleanEntryWithBlankValues;
/// Scroll Bar Orientation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Horiz.
pub type HorizontalScrollBar = BooleanEntryWithBlankValues;
/// ActiveX Control.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:MapOCX.
pub type MapOcxControl = BooleanEntryWithBlankValues;
/// Camera Tool.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Camera.
pub type CameraObject = BooleanEntryWithBlankValues;
/// Recalculation Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:RecalcAlways.
pub type RecalculateAlways = BooleanEntryWithBlankValues;
/// Font AutoScale.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:AutoScale.
pub type AutoScaleFont = BooleanEntryWithBlankValues;
/// Dynamic Data Exchange.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:DDE.
pub type DdeObject = BooleanEntryWithBlankValues;
/// UI Object Toggle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:UIObj.
pub type UiObject = BooleanEntryWithBlankValues;
/// Anchor.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Anchor.
pub type Anchor = crate::simple_type::StringValue;
/// Horizontal Text Alignment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:TextHAlign.
pub type HorizontalTextAlignment = crate::simple_type::StringValue;
/// Vertical Text Alignment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:TextVAlign.
pub type VerticalTextAlignment = crate::simple_type::StringValue;
/// List Items Source Range.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaRange.
pub type FormulaRange = crate::simple_type::StringValue;
/// Selection Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:SelType.
pub type SelectionType = crate::simple_type::StringValue;
/// Multiple Selections.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:MultiSel.
pub type MultiSelections = crate::simple_type::StringValue;
/// Callback Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:LCT.
pub type ListBoxCallbackType = crate::simple_type::StringValue;
/// Non-linked List Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:ListItem.
pub type ListItem = crate::simple_type::StringValue;
/// Dropdown Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:DropStyle.
pub type DropStyle = crate::simple_type::StringValue;
/// Linked Formula.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaLink.
pub type FormulaLink = crate::simple_type::StringValue;
/// Camera Source Range.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaPict.
pub type FormulaPicture = crate::simple_type::StringValue;
/// Linked Formula - Group Box.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaGroup.
pub type FormulaGroup = crate::simple_type::StringValue;
/// HTML Script Text.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:ScriptText.
pub type ScriptText = crate::simple_type::StringValue;
/// HTML Script Attributes.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:ScriptExtended.
pub type ScriptExtended = crate::simple_type::StringValue;
/// Text Formula.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaTxbx.
pub type FormulaTextBox = crate::simple_type::StringValue;
/// Reference to Custom Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:FmlaMacro.
pub type FormulaMacro = crate::simple_type::StringValue;
/// Primary Keyboard Accelerator.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Accel.
pub type AcceleratorPrimary = crate::simple_type::ByteValue;
/// Secondary Keyboard Accelerator.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Accel2.
pub type AcceleratorSecondary = crate::simple_type::ByteValue;
/// Comment Row Target.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Row.
pub type CommentRowTarget = crate::simple_type::IntegerValue;
/// Comment Column Target.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Column.
pub type CommentColumnTarget = crate::simple_type::IntegerValue;
/// Validation Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:VTEdit.
pub type InputValidationType = crate::simple_type::IntegerValue;
/// Minimum Width.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:WidthMin.
pub type MinDropDownWidth = crate::simple_type::IntegerValue;
/// Selected Entry.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Sel.
pub type SelectionEntry = crate::simple_type::IntegerValue;
/// Dropdown Maximum Lines.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:DropLines.
pub type DropLines = crate::simple_type::IntegerValue;
/// Checked.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Checked.
pub type Checked = crate::simple_type::IntegerValue;
/// Scroll bar position.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Val.
pub type ScrollBarPosition = crate::simple_type::IntegerValue;
/// Scroll Bar Minimum.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Min.
pub type ScrollBarMin = crate::simple_type::IntegerValue;
/// Scroll Bar Maximum.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Max.
pub type ScrollBarMax = crate::simple_type::IntegerValue;
/// Scroll Bar Increment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Inc.
pub type ScrollBarIncrement = crate::simple_type::IntegerValue;
/// Scroll Bar Page Increment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Page.
pub type ScrollBarPageIncrement = crate::simple_type::IntegerValue;
/// Scroll Bar Width.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:Dx.
pub type ScrollBarWidth = crate::simple_type::IntegerValue;
/// Clipboard Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:CF.
pub type ClipboardFormat = ClipboardFormatValues;
/// HTML Script Language.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:ScriptLanguage.
pub type ScriptLanguage = crate::simple_type::IntegerValue;
/// HTML Script Location.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xvml:ScriptLocation.
pub type ScriptLocation = crate::simple_type::IntegerValue;
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ClientDataChoice {
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:MoveWithCells"))]
  XvmlMoveWithCells(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:SizeWithCells"))]
  XvmlSizeWithCells(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:string/xvml:Anchor"))]
  XvmlAnchor(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Locked"))]
  XvmlLocked(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:DefaultSize"))]
  XvmlDefaultSize(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:PrintObject"))]
  XvmlPrintObject(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Disabled"))]
  XvmlDisabled(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:AutoFill"))]
  XvmlAutoFill(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:AutoLine"))]
  XvmlAutoLine(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:AutoPict"))]
  XvmlAutoPict(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_Macro/xvml:FmlaMacro"))]
  XvmlFmlaMacro(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/xvml:TextHAlign"))]
  XvmlTextHAlign(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/xvml:TextVAlign"))]
  XvmlTextVAlign(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:LockText"))]
  XvmlLockText(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:JustLastX"))]
  XvmlJustLastX(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:SecretEdit"))]
  XvmlSecretEdit(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Default"))]
  XvmlDefault(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Help"))]
  XvmlHelp(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Cancel"))]
  XvmlCancel(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Dismiss"))]
  XvmlDismiss(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:unsignedByte/xvml:Accel"))]
  XvmlAccel(crate::simple_type::ByteValue),
  #[sdk(text_child(qname = "xsd:unsignedByte/xvml:Accel2"))]
  XvmlAccel2(crate::simple_type::ByteValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Row"))]
  XvmlRow(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Column"))]
  XvmlColumn(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Visible"))]
  XvmlVisible(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:RowHidden"))]
  XvmlRowHidden(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:ColHidden"))]
  XvmlColHidden(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:integer/xvml:VTEdit"))]
  XvmlVtEdit(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:MultiLine"))]
  XvmlMultiLine(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:VScroll"))]
  XvmlVScroll(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:ValidIds"))]
  XvmlValidIds(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaRange"))]
  XvmlFmlaRange(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:WidthMin"))]
  XvmlWidthMin(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Sel"))]
  XvmlSel(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:NoThreeD2"))]
  XvmlNoThreeD2(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:string/xvml:SelType"))]
  XvmlSelType(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/xvml:MultiSel"))]
  XvmlMultiSel(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/xvml:LCT"))]
  XvmlLct(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/xvml:ListItem"))]
  XvmlListItem(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/xvml:DropStyle"))]
  XvmlDropStyle(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Colored"))]
  XvmlColored(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:integer/xvml:DropLines"))]
  XvmlDropLines(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Checked"))]
  XvmlChecked(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaLink"))]
  XvmlFmlaLink(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaPict"))]
  XvmlFmlaPict(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:NoThreeD"))]
  XvmlNoThreeD(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:FirstButton"))]
  XvmlFirstButton(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaGroup"))]
  XvmlFmlaGroup(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Val"))]
  XvmlVal(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Min"))]
  XvmlMin(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Max"))]
  XvmlMax(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Inc"))]
  XvmlInc(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:integer/xvml:Page"))]
  XvmlPage(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Horiz"))]
  XvmlHoriz(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:integer/xvml:Dx"))]
  XvmlDx(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:MapOCX"))]
  XvmlMapOcx(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_CF/xvml:CF"))]
  XvmlCf(ClipboardFormatValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:Camera"))]
  XvmlCamera(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:RecalcAlways"))]
  XvmlRecalcAlways(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:AutoScale"))]
  XvmlAutoScale(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:DDE"))]
  XvmlDde(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xvml:ST_TrueFalseBlank/xvml:UIObj"))]
  XvmlUiObj(BooleanEntryWithBlankValues),
  #[sdk(text_child(qname = "xsd:string/xvml:ScriptText"))]
  XvmlScriptText(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:string/xvml:ScriptExtended"))]
  XvmlScriptExtended(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:nonNegativeInteger/xvml:ScriptLanguage"))]
  XvmlScriptLanguage(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:nonNegativeInteger/xvml:ScriptLocation"))]
  XvmlScriptLocation(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:string/xvml:FmlaTxbx"))]
  XvmlFmlaTxbx(crate::simple_type::StringValue),
}
