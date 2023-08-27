// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ItemTypeWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class ItemTypeWindowClass : WindowClass
  {
     ItemTypeListId: i32;
     ListClass ItemTypeListObj;
     BAddItemTypeId: i32;
     BAddItemTypeTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     int[] Bgameslotid;
     int[] Bgameslottextid;
     int[] Bregimeslotid;
     int[] Bregimeslottextid;
     int[] Bregimeslotcostid;
     int[] Bregimeslotcosttextid;
     int[] Bitemid;
     int[] bitemtextid;
     int[] BresId;
     int[] BResTextId;
     BRemoveItemTypeId: i32;
     BRemoveItemTypeTextId: i32;
     BRemoveItemType2Id: i32;
     BRemoveItemTypeText2Id: i32;
     BCloneItemTypeId: i32;
     BCloneItemTypeTextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     B7Id: i32;
     B7TextId: i32;
     B8Id: i32;
     B8TextId: i32;
     B9Id: i32;
     B9TextId: i32;
     B10Id: i32;
     B10TextId: i32;
     B11Id: i32;
     B11TextId: i32;
     B12Id: i32;
     B12TextId: i32;
     B13Id: i32;
     B13TextId: i32;
     B14Id: i32;
     B14TextId: i32;
     B15Id: i32;
     B15TextId: i32;
     B16Id: i32;
     B16TextId: i32;
     B17Id: i32;
     B17TextId: i32;
     B18Id: i32;
     B18TextId: i32;
     B19Id: i32;
     B19TextId: i32;
     B20Id: i32;
     B20TextId: i32;
     PGListId: i32;
     ListClass PGListObj;
     B3bId: i32;
     B3bTextId: i32;
     ItemTypeNr: i32;
     detailnr: i32;
     ss: String;

    pub ItemTypeWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Item Types")
    {
      this.Bgameslotid = new int[5];
      this.Bgameslottextid = new int[5];
      this.Bregimeslotid = new int[5];
      this.Bregimeslottextid = new int[5];
      this.Bregimeslotcostid = new int[5];
      this.Bregimeslotcosttextid = new int[5];
      this.Bitemid = new int[5];
      this.bitemtextid = new int[5];
      this.BresId = new int[5];
      this.BResTextId = new int[5];
      this.ItemTypeNr = -1;
      this.detailnr = -1;
      this.MakeItemTypeListGUI(-1);
    }

    pub fn DoRefresh() => this.MakeItemTypeTypeItemGUI();

     void MakeItemTypeListGUI(tItemTypenr: i32)
    {
      if (this.ItemTypeListId > 0)
        this.RemoveSubPart(this.ItemTypeListId);
      SubPartClass tsubpart;
      if (this.game.Data.ItemTypeCounter > -1)
      {
        this.ItemTypeListObj = ListClass::new();
        let mut itemTypeCounter: i32 =  this.game.Data.ItemTypeCounter;
        for (let mut index: i32 =  0; index <= itemTypeCounter; index += 1)
          this.ItemTypeListObj.add(Strings.Trim(Conversion.Str( index)) + ") " + this.game.Data.ItemTypeObj[index].Name, index);
        ListClass itemTypeListObj = this.ItemTypeListObj;
        let mut tlistselect: i32 =  tItemTypenr;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart =  new ListSubPartClass(itemTypeListObj, 9, 200, tlistselect, game, tHeader: "Item Types", tbackbitmap: ( local1), bbx: 10, bby: 50, overruleFont: ( local2));
        this.ItemTypeListId = this.AddSubPart( tsubpart, 10, 50, 200, 192, 0);
        this.ItemTypeNr = tItemTypenr;
        this.MakeItemTypeTypeItemGUI();
      }
      else
      {
        this.ItemTypeNr = tItemTypenr;
        this.MakeItemTypeTypeItemGUI();
      }
      if (this.BAddItemTypeId > 0)
        this.RemoveSubPart(this.BAddItemTypeId);
      if (this.BAddItemTypeTextId > 0)
        this.RemoveSubPart(this.BAddItemTypeTextId);
      this.ss = "Click to add an itemtype to the list";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.BAddItemTypeId = this.AddSubPart( tsubpart, 10, 270, 32, 16, 1);
      }
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  TextPartClass::new("Add a ItemType", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.BAddItemTypeTextId = this.AddSubPart( tsubpart, 50, 269, 300, 20, 0);
      }
      if (this.B13Id > 0)
        this.RemoveSubPart(this.B13Id);
      if (this.B13TextId > 0)
        this.RemoveSubPart(this.B13TextId);
      this.ss = "Click to set all itemtypes to be non produce-able by all peoplegroups.";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
        this.B13Id = this.AddSubPart( tsubpart, 10, 250, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Set All to false", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B13TextId = this.AddSubPart( tsubpart, 50, 249, 400, 20, 0);
    }

     void MakeItemTypeTypeItemGUI()
    {
      if (this.BNameId > 0)
        this.RemoveSubPart(this.BNameId);
      if (this.BNameTextId > 0)
        this.RemoveSubPart(this.BNameTextId);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.B3TextId > 0)
        this.RemoveSubPart(this.B3TextId);
      if (this.B3bId > 0)
        this.RemoveSubPart(this.B3bId);
      if (this.B3bTextId > 0)
        this.RemoveSubPart(this.B3bTextId);
      if (this.PGListId > 0)
        this.RemoveSubPart(this.PGListId);
      if (this.B4Id > 0)
        this.RemoveSubPart(this.B4Id);
      if (this.B4TextId > 0)
        this.RemoveSubPart(this.B4TextId);
      if (this.B5Id > 0)
        this.RemoveSubPart(this.B5Id);
      if (this.B5TextId > 0)
        this.RemoveSubPart(this.B5TextId);
      if (this.B6Id > 0)
        this.RemoveSubPart(this.B6Id);
      if (this.B6TextId > 0)
        this.RemoveSubPart(this.B6TextId);
      if (this.B7Id > 0)
        this.RemoveSubPart(this.B7Id);
      if (this.B7TextId > 0)
        this.RemoveSubPart(this.B7TextId);
      if (this.B8Id > 0)
        this.RemoveSubPart(this.B8Id);
      if (this.B8TextId > 0)
        this.RemoveSubPart(this.B8TextId);
      if (this.B9Id > 0)
        this.RemoveSubPart(this.B9Id);
      if (this.B9TextId > 0)
        this.RemoveSubPart(this.B9TextId);
      if (this.B10Id > 0)
        this.RemoveSubPart(this.B10Id);
      if (this.B10TextId > 0)
        this.RemoveSubPart(this.B10TextId);
      if (this.B11Id > 0)
        this.RemoveSubPart(this.B11Id);
      if (this.B11TextId > 0)
        this.RemoveSubPart(this.B11TextId);
      if (this.B12Id > 0)
        this.RemoveSubPart(this.B12Id);
      if (this.B12TextId > 0)
        this.RemoveSubPart(this.B12TextId);
      if (this.B14Id > 0)
        this.RemoveSubPart(this.B14Id);
      if (this.B14TextId > 0)
        this.RemoveSubPart(this.B14TextId);
      if (this.B15Id > 0)
        this.RemoveSubPart(this.B15Id);
      if (this.B15TextId > 0)
        this.RemoveSubPart(this.B15TextId);
      if (this.B16Id > 0)
        this.RemoveSubPart(this.B16Id);
      if (this.B16TextId > 0)
        this.RemoveSubPart(this.B16TextId);
      if (this.B17Id > 0)
        this.RemoveSubPart(this.B17Id);
      if (this.B17TextId > 0)
        this.RemoveSubPart(this.B17TextId);
      if (this.B18Id > 0)
        this.RemoveSubPart(this.B18Id);
      if (this.B18TextId > 0)
        this.RemoveSubPart(this.B18TextId);
      if (this.B19Id > 0)
        this.RemoveSubPart(this.B19Id);
      if (this.B19TextId > 0)
        this.RemoveSubPart(this.B19TextId);
      if (this.B20Id > 0)
        this.RemoveSubPart(this.B20Id);
      if (this.B20TextId > 0)
        this.RemoveSubPart(this.B20TextId);
      let mut index1: i32 =  0;
      do
      {
        if (this.Bgameslotid[index1] > 0)
          this.RemoveSubPart(this.Bgameslotid[index1]);
        if (this.Bgameslottextid[index1] > 0)
          this.RemoveSubPart(this.Bgameslottextid[index1]);
        if (this.Bregimeslotid[index1] > 0)
          this.RemoveSubPart(this.Bregimeslotid[index1]);
        if (this.Bregimeslottextid[index1] > 0)
          this.RemoveSubPart(this.Bregimeslottextid[index1]);
        if (this.Bregimeslotcostid[index1] > 0)
          this.RemoveSubPart(this.Bregimeslotcostid[index1]);
        if (this.Bregimeslotcosttextid[index1] > 0)
          this.RemoveSubPart(this.Bregimeslotcosttextid[index1]);
        if (this.Bitemid[index1] > 0)
          this.RemoveSubPart(this.Bitemid[index1]);
        if (this.bitemtextid[index1] > 0)
          this.RemoveSubPart(this.bitemtextid[index1]);
        if (this.BresId[index1] > 0)
          this.RemoveSubPart(this.BresId[index1]);
        if (this.BResTextId[index1] > 0)
          this.RemoveSubPart(this.BResTextId[index1]);
        index1 += 1;
      }
      while (index1 <= 4);
      if (this.BRemoveItemTypeId > 0)
        this.RemoveSubPart(this.BRemoveItemTypeId);
      if (this.BRemoveItemTypeTextId > 0)
        this.RemoveSubPart(this.BRemoveItemTypeTextId);
      if (this.BCloneItemTypeId > 0)
        this.RemoveSubPart(this.BCloneItemTypeId);
      if (this.BCloneItemTypeTextId > 0)
        this.RemoveSubPart(this.BCloneItemTypeTextId);
      if (this.BRemoveItemType2Id > 0)
        this.RemoveSubPart(this.BRemoveItemType2Id);
      if (this.BRemoveItemTypeText2Id > 0)
        this.RemoveSubPart(this.BRemoveItemTypeText2Id);
      if (this.ItemTypeNr <= -1)
        return;
      this.ss = "Click to change the name of this itemtype";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BNameId = this.AddSubPart( tsubpart, 370, 50, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Name: " + this.game.Data.ItemTypeObj[this.ItemTypeNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.BNameTextId = this.AddSubPart( tsubpart1, 410, 49, 400, 20, 0);
      this.ss = "Click to toggle on/off if this is a supply point";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B1Id = this.AddSubPart( tsubpart2, 10, 610, 32, 16, 1);
      }
      let mut tsubpart3: SubPartClass =  TextPartClass::new("IsSupply: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].IsSupply), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.B1TextId = this.AddSubPart( tsubpart3, 50, 609, 200, 20, 0);
      this.ss = "Click to set the itemtypegroup this itemtype belongs to";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B2Id = this.AddSubPart( tsubpart3, 370, 70, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("ItemType Group #: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].ItemGroup), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B2TextId = this.AddSubPart( tsubpart3, 410, 69, 400, 20, 0);
      let mut index2: i32 =  0;
      do
      {
        this.ss = "Click to set a minimum value that a certain gameslot has to have in order to be able to produce this itemtype. -1=none needed";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          int[] bgameslotid = this.Bgameslotid;
          let mut index3: i32 =  index2;
          tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          let mut num: i32 =  this.AddSubPart( tsubpart3, 370, 90 + index2 * 20, 32, 16, 1);
          bgameslotid[index3] = num;
        }
        txt1: String;
        if (this.game.Data.ItemTypeObj[this.ItemTypeNr].GameSlotsNeeded[index2] > -1)
          txt1 = "GameSlot: " + this.game.Data.GameSlotName[this.game.Data.ItemTypeObj[this.ItemTypeNr].GameSlotsNeeded[index2]] + "(" + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].GameSlotsNeeded[index2]) + ") = " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].GameSlotsNeededQty[index2]);
        else
          txt1 = "No GameSlot Needed";
        int[] bgameslottextid = this.Bgameslottextid;
        let mut index4: i32 =  index2;
        tsubpart3 =  TextPartClass::new(txt1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        let mut num1: i32 =  this.AddSubPart( tsubpart3, 410, 90 + index2 * 20 - 1, 400, 20, 0);
        bgameslottextid[index4] = num1;
        this.ss = "Click to set a minimum value that a certain regimeslot has to have in order to be able to produce this itemtype. -1=none needed";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          int[] bregimeslotid = this.Bregimeslotid;
          let mut index5: i32 =  index2;
          tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          let mut num2: i32 =  this.AddSubPart( tsubpart3, 370, 190 + index2 * 20, 32, 16, 1);
          bregimeslotid[index5] = num2;
        }
        txt2: String;
        if (this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsNeeded[index2] > -1)
          txt2 = "RegimeSlot: " + this.game.Data.RegimeSlotName[this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsNeeded[index2]] + "(" + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsNeeded[index2]) + ") = " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsNeededQty[index2]);
        else
          txt2 = "No RegimeSlot Needed";
        int[] bregimeslottextid = this.Bregimeslottextid;
        let mut index6: i32 =  index2;
        tsubpart3 =  TextPartClass::new(txt2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        let mut num3: i32 =  this.AddSubPart( tsubpart3, 410, 190 + index2 * 20 - 1, 400, 20, 0);
        bregimeslottextid[index6] = num3;
        this.ss = "Click to set a howmuch points from regime variable X this itemtype needs to USE in order to produce it. -1=none needed";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          int[] bregimeslotcostid = this.Bregimeslotcostid;
          let mut index7: i32 =  index2;
          tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          let mut num4: i32 =  this.AddSubPart( tsubpart3, 410, 300 + index2 * 20, 32, 16, 1);
          bregimeslotcostid[index7] = num4;
        }
        txt3: String;
        if (this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsCost[index2] > -1)
          txt3 = "RegimeSlotCost: " + this.game.Data.RegimeSlotName[this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsCost[index2]] + "(" + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsCost[index2]) + ") = " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsCostQty[index2]);
        else
          txt3 = "No RegimeSlotCost Needed";
        int[] bregimeslotcosttextid = this.Bregimeslotcosttextid;
        let mut index8: i32 =  index2;
        tsubpart3 =  TextPartClass::new(txt3, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        let mut num5: i32 =  this.AddSubPart( tsubpart3, 450, 300 + index2 * 20 - 1, 400, 20, 0);
        bregimeslotcosttextid[index8] = num5;
        this.ss = "Click to set a researchfield that is neccessary to produce this itemtype";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          int[] bresId = this.BresId;
          let mut index9: i32 =  index2;
          tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          let mut num6: i32 =  this.AddSubPart( tsubpart3, 370, 450 + index2 * 20, 32, 16, 1);
          bresId[index9] = num6;
        }
        txt4: String;
        if (this.game.Data.ItemTypeObj[this.ItemTypeNr].ResFieldNeeded[index2] > -1)
          txt4 = "ResField: " + this.game.Data.ResearchObj[this.game.Data.ItemTypeObj[this.ItemTypeNr].ResFieldNeeded[index2]].Name + "(" + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].ResFieldNeeded[index2]) + ") ";
        else
          txt4 = "No Research needed";
        int[] bresTextId = this.BResTextId;
        let mut index10: i32 =  index2;
        tsubpart3 =  TextPartClass::new(txt4, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        let mut num7: i32 =  this.AddSubPart( tsubpart3, 410, 450 + index2 * 20 - 1, 400, 20, 0);
        bresTextId[index10] = num7;
        index2 += 1;
      }
      while (index2 <= 4);
      this.ss = "Click to set the production cost of this itemtype in prodpoints";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B5Id = this.AddSubPart( tsubpart3, 370, 430, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("Production Cost: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].ProdWeight), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B5TextId = this.AddSubPart( tsubpart3, 410, 429, 400, 20, 0);
      this.ss = "Click to toggle on/off if this itemtype is a Political Point";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B8Id = this.AddSubPart( tsubpart3, 370, 550, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("IsPolPt: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].IsResPt), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B8TextId = this.AddSubPart( tsubpart3, 410, 549, 400, 20, 0);
      this.ss = "Click to set a possible production multiplier. Or the ammount of this polpt/supply/sftype that is produced when 1 instance of this itemtype is produced.";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B9Id = this.AddSubPart( tsubpart3, 370, 570, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("ProdMultiplier: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].Multiplier), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B9TextId = this.AddSubPart( tsubpart3, 410, 569, 400, 20, 0);
      this.ss = "Click to set the SFType that is created when this itemtype is produced. -1=no sftype";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B10Id = this.AddSubPart( tsubpart3, 370, 590, 32, 16, 1);
      }
      if (this.game.Data.ItemTypeObj[this.ItemTypeNr].IsSFType == -1)
      {
        tsubpart3 =  TextPartClass::new("IsSFType#: -1", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.B10TextId = this.AddSubPart( tsubpart3, 410, 589, 400, 20, 0);
      }
      else if (this.game.Data.ItemTypeObj[this.ItemTypeNr].IsSFType <= this.game.Data.SFTypeCounter)
      {
        tsubpart3 =  TextPartClass::new("IsSFType#: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].IsSFType) + "," + this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[this.ItemTypeNr].IsSFType].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.B10TextId = this.AddSubPart( tsubpart3, 410, 589, 400, 20, 0);
      }
      this.ss = "Click to set which itemtype should be blocked from the production list if it is possible to produce this itemtype";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B11Id = this.AddSubPart( tsubpart3, 370, 610, 32, 16, 1);
      }
      if (this.game.Data.ItemTypeObj[this.ItemTypeNr].Blocks > -1)
      {
        tsubpart3 =  TextPartClass::new("Blocks: " + this.game.Data.ItemTypeObj[this.game.Data.ItemTypeObj[this.ItemTypeNr].Blocks].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.B11TextId = this.AddSubPart( tsubpart3, 410, 609, 400, 20, 0);
      }
      else
      {
        tsubpart3 =  TextPartClass::new("Blocks: -1", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.B11TextId = this.AddSubPart( tsubpart3, 410, 609, 400, 20, 0);
      }
      this.ss = "Click to set if producing this itemtype adds 1 point (*multiplier) to a certain regimeslot";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B12Id = this.AddSubPart( tsubpart3, 370, 630, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("IsRegimeSlot: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].IsRegimeSlot), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B12TextId = this.AddSubPart( tsubpart3, 410, 629, 400, 20, 0);
      this.ss = "Click to set if graphics of itemtype should be overruled by specific SFType (dummies can be made)";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B14Id = this.AddSubPart( tsubpart3, 370, 650, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("UseSFTypeGraphic: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].UseSFType), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B14TextId = this.AddSubPart( tsubpart3, 410, 649, 400, 20, 0);
      this.ss = "extra XP points +/-";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B15Id = this.AddSubPart( tsubpart3, 810, 110, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("XpMod: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].XpMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B15TextId = this.AddSubPart( tsubpart3, 850, 109, 400, 20, 0);
      this.ss = "extra morale points +/-";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B16Id = this.AddSubPart( tsubpart3, 810, 130, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("MorMod: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].MorMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B16TextId = this.AddSubPart( tsubpart3, 850, 129, 400, 20, 0);
      this.ss = "-1=no mod";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B17Id = this.AddSubPart( tsubpart3, 810, 150, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("PeopleMod: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].PeopleMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B17TextId = this.AddSubPart( tsubpart3, 850, 149, 400, 20, 0);
      this.ss = "-1=no mod";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B18Id = this.AddSubPart( tsubpart3, 810, 170, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("MoveTypeMod: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].MoveTypeMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B18TextId = this.AddSubPart( tsubpart3, 850, 169, 400, 20, 0);
      this.ss = "Set Regime Specific setting. -1=all. >-1 = a specific regime only. -2= none";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B19Id = this.AddSubPart( tsubpart3, 810, 190, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("RegimeSpecific: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSpecific), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B19TextId = this.AddSubPart( tsubpart3, 850, 189, 400, 20, 0);
      this.ss = "Set Prod Mod to use (1,2,3,4) (old 0=seen as 1)";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B20Id = this.AddSubPart( tsubpart3, 810, 210, 32, 16, 1);
      }
      tsubpart3 =  TextPartClass::new("UseProdMod: " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].UseProdMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
      this.B20TextId = this.AddSubPart( tsubpart3, 850, 209, 400, 20, 0);
      this.ss = "Click to remove this itemtype";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.BRemoveItemTypeId = this.AddSubPart( tsubpart3, 10, 290, 32, 16, 1);
      }
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  TextPartClass::new("Remove this ItemType", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.BRemoveItemTypeTextId = this.AddSubPart( tsubpart3, 50, 289, 200, 20, 0);
      }
      this.ss = "Copy this itemtype (put at end of list)";
      if (this.ItemTypeNr > -1)
      {
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
          this.BCloneItemTypeId = this.AddSubPart( tsubpart3, 10, 310, 32, 16, 1);
        }
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  TextPartClass::new("Copy this ItemType", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.BCloneItemTypeTextId = this.AddSubPart( tsubpart3, 50, 309, 200, 20, 0);
        }
      }
      this.ss = "Click to remove ALL itemtypes";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.BRemoveItemType2Id = this.AddSubPart( tsubpart3, 10, 330, 32, 16, 1);
      }
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart3 =  TextPartClass::new("Remove ALL", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.BRemoveItemTypeText2Id = this.AddSubPart( tsubpart3, 50, 329, 200, 20, 0);
      }
      this.PGListObj = ListClass::new();
      if (this.detailnr < -1 | this.detailnr > 99)
        this.detailnr = -1;
      let mut index11: i32 =  0;
      do
      {
        this.PGListObj.add(Conversion.Str( index11) + ") " + this.game.Data.TempString[index11 + 200] + " = " + Conversion.Str( this.game.Data.ItemTypeObj[this.ItemTypeNr].PeopleGroup[index11]), index11);
        index11 += 1;
      }
      while (index11 <= 99);
      ListClass pgListObj = this.PGListObj;
      let mut detailnr: i32 =  this.detailnr;
      let mut game: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      tsubpart3 =  new ListSubPartClass(pgListObj, 6, 200, detailnr, game, tHeader: "Who must own?", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
      this.PGListId = this.AddSubPart( tsubpart3, 10, 350, 200, 144, 0);
      this.maketabsheet3b();
    }

     void maketabsheet3b()
    {
      if (this.detailnr <= -1)
        return;
      this.ss = "Click to toggle on/off if this peoplegroup has to own the production location in order to build this itemtype";
      if (this.B3bId > 0)
        this.RemoveSubPart(this.B3bId);
      if (this.B3bTextId > 0)
        this.RemoveSubPart(this.B3bTextId);
      if (Strings.Len(this.game.Data.MasterFile) == 0 | !this.game.Data.MasterfileReadPeople)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.B3bId = this.AddSubPart( tsubpart, 215, 350, 32, 16, 1);
      }
      if (!(Strings.Len(this.game.Data.MasterFile) == 0 | !this.game.Data.MasterfileReadPeople))
        return;
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Change", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: this.ss);
      this.B3bTextId = this.AddSubPart( tsubpart1, 250, 349, 200, 20, 0);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
label_106:
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.ItemTypeListId)
            {
              let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                this.ItemTypeNr = num2;
                this.MakeItemTypeTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BAddItemTypeId)
            {
              this.game.Data.AddItemType();
              this.MakeItemTypeListGUI(this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BNameId)
            {
              this.game.Data.ItemTypeObj[this.ItemTypeNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              this.MakeItemTypeListGUI(this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B1Id)
            {
              this.game.Data.ItemTypeObj[this.ItemTypeNr].IsSupply = !this.game.Data.ItemTypeObj[this.ItemTypeNr].IsSupply;
              this.MakeItemTypeListGUI(this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B2Id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 17, this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B5Id)
            {
              let mut num3: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new prodweight, please.", "Shadow Empire : Planetary Conquest")));
              if (num3 < 0 | num3 > 99999)
              {
                let mut num4: i32 =   Interaction.MsgBox( "Between 0 and 99999 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.ItemTypeObj[this.ItemTypeNr].ProdWeight = num3;
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B13Id)
            {
              let mut itemTypeCounter: i32 =  this.game.Data.ItemTypeCounter;
              for (let mut index2: i32 =  0; index2 <= itemTypeCounter; index2 += 1)
              {
                let mut index3: i32 =  0;
                do
                {
                  this.game.Data.ItemTypeObj[index2].PeopleGroup[index3] = false;
                  index3 += 1;
                }
                while (index3 <= 99);
              }
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B8Id)
            {
              this.game.Data.ItemTypeObj[this.ItemTypeNr].IsResPt = !this.game.Data.ItemTypeObj[this.ItemTypeNr].IsResPt;
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B9Id)
            {
              let mut num5: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new multiplier, please.", "Shadow Empire : Planetary Conquest")));
              if (num5 < 1 | num5 > 99999)
              {
                let mut num6: i32 =   Interaction.MsgBox( "Between 1 and 99999 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.ItemTypeObj[this.ItemTypeNr].Multiplier = num5;
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B10Id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 19, this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B14Id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 34, this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B11Id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 20, this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B12Id)
            {
              let mut num7: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give which regime slot is raised +1 please.", "Shadow Empire : Planetary Conquest")));
              if (num7 < -1 | num7 > 499)
              {
                let mut num8: i32 =   Interaction.MsgBox( "Between -1 and 499 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.ItemTypeObj[this.ItemTypeNr].IsRegimeSlot = num7;
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BRemoveItemTypeId)
            {
              this.game.Data.RemoveItemType(this.ItemTypeNr);
              if (this.ItemTypeNr > this.game.Data.ItemTypeCounter)
                --this.ItemTypeNr;
              this.MakeItemTypeListGUI(this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BCloneItemTypeId)
            {
              this.game.Data.AddItemType();
              this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter] = this.game.Data.ItemTypeObj[this.ItemTypeNr].Clone();
              this.ItemTypeNr = this.game.Data.ItemTypeCounter;
              this.MakeItemTypeListGUI(this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B15Id)
            {
              let mut num9: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give XP Modifier", "Shadow Empire : Planetary Conquest")));
              if (num9 < -999 | num9 > 999)
              {
                let mut num10: i32 =   Interaction.MsgBox( "Between -999 and 999 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.ItemTypeObj[this.ItemTypeNr].XpMod = num9;
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B19Id)
            {
              let mut num11: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Regime Specific (-2=none,-1=all,>-1=reg)", "Shadow Empire : Planetary Conquest")));
              if (num11 <= this.game.Data.RegimeCounter | num11 >= -2)
              {
                this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSpecific = num11;
              }
              else
              {
                let mut num12: i32 =   Interaction.MsgBox( "Between -2 and regimecount please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B20Id)
            {
              let mut num13: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Use Prod Mod (1-4). 0=seen as 1", "Shadow Empire : Planetary Conquest")));
              if (num13 <= 4 | num13 >= 1)
              {
                this.game.Data.ItemTypeObj[this.ItemTypeNr].UseProdMod = num13;
              }
              else
              {
                let mut num14: i32 =   Interaction.MsgBox( "Between 1 and 4 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B16Id)
            {
              let mut num15: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Mor Modifier", "Shadow Empire : Planetary Conquest")));
              if (num15 < -999 | num15 > 999)
              {
                let mut num16: i32 =   Interaction.MsgBox( "Between -999 and 999 please. ", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                this.game.Data.ItemTypeObj[this.ItemTypeNr].MorMod = num15;
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B17Id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 49, this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B18Id)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 50, this.ItemTypeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BRemoveItemType2Id)
            {
              for (let mut itemTypeCounter: i32 =  this.game.Data.ItemTypeCounter; itemTypeCounter >= 0; itemTypeCounter += -1)
                this.game.Data.RemoveItemType(itemTypeCounter);
              this.ItemTypeNr = -1;
              this.MakeItemTypeListGUI(-1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.PGListId)
            {
              let mut num17: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num17 > -1)
              {
                this.detailnr = num17;
                this.MakeItemTypeTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.B3bId)
            {
              this.game.Data.ItemTypeObj[this.ItemTypeNr].PeopleGroup[this.detailnr] = !this.game.Data.ItemTypeObj[this.ItemTypeNr].PeopleGroup[this.detailnr];
              this.MakeItemTypeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            let mut tnr2: i32 =  0;
            while (this.SubPartID[index1] != this.Bgameslotid[tnr2])
            {
              if (this.SubPartID[index1] == this.Bregimeslotid[tnr2])
              {
                let mut num18: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("TYPE: Give new regimeslot# needed", "Shadow Empire : Planetary Conquest")));
                let mut num19: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("QTY: Give new value needed for this slot", "Shadow Empire : Planetary Conquest")));
                if (num18 < -1 | num18 > 499)
                {
                  let mut num20: i32 =   Interaction.MsgBox( "Regime Slot # Between -1 and 499 please..", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsNeeded[tnr2] = num18;
                  this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsNeededQty[tnr2] = num19;
                }
                this.MakeItemTypeTypeItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (this.SubPartID[index1] == this.Bregimeslotcostid[tnr2])
              {
                let mut num21: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("TYPE: Give new regimeslot# needed", "Shadow Empire : Planetary Conquest")));
                let mut num22: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("QTY: Give new ammount of points needed from this regimeslot in order to produce", "Shadow Empire : Planetary Conquest")));
                if (num21 < -1 | num21 > 499 & num22 > 0)
                {
                  let mut num23: i32 =   Interaction.MsgBox( "Regime Slot # Between -1 and 499 please.. and qty>0", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsCost[tnr2] = num21;
                  this.game.Data.ItemTypeObj[this.ItemTypeNr].RegimeSlotsCostQty[tnr2] = num22;
                }
                this.MakeItemTypeTypeItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (this.SubPartID[index1] == this.BresId[tnr2])
              {
                Form3::new( this.formref).Initialize(this.game.Data, 18, this.ItemTypeNr, tnr2);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              tnr2 += 1;
              if (tnr2 > 4)
                goto label_106;
            }
            let mut num24: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("TYPE: Give new gameslot# needed", "Shadow Empire : Planetary Conquest")));
            let mut num25: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("QTY: Give new value needed for this slot", "Shadow Empire : Planetary Conquest")));
            if (num24 < -1 | num24 > 499)
            {
              let mut num26: i32 =   Interaction.MsgBox( "Game slot # Between -1 and 499 please..", Title: ( "Shadow Empire : Planetary Conquest"));
            }
            else
            {
              this.game.Data.ItemTypeObj[this.ItemTypeNr].GameSlotsNeeded[tnr2] = num24;
              this.game.Data.ItemTypeObj[this.ItemTypeNr].GameSlotsNeededQty[tnr2] = num25;
            }
            this.MakeItemTypeTypeItemGUI();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
