// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LandscapeTypeWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class LandscapeTypeWindowClass : WindowClass
  {
     LTListId: i32;
     ListClass LTListObj;
     BAddLTId: i32;
     BAddLTTextId: i32;
     BCloneLTId: i32;
     BCloneLTTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     TxtId: i32;
     TxtStrId: i32;
     BSLId: i32;
     BSLTextId: i32;
     BSL2Id: i32;
     BSL2TextId: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     BRemoveLtId: i32;
     BRemoveLtTextId: i32;
     BBuildRoadId: i32;
     BBuildRoadTextId: i32;
     e1id: i32;
     e1textid: i32;
     e2id: i32;
     e2textid: i32;
     e3id: i32;
     e3textid: i32;
     e4id: i32;
     e4textid: i32;
     e5id: i32;
     e5textid: i32;
     e6id: i32;
     e6textid: i32;
     e7id: i32;
     e7textid: i32;
     e7bid: i32;
     e7btextid: i32;
     e8id: i32;
     e8textid: i32;
     e9id: i32;
     e9textid: i32;
     e10id: i32;
     e10textid: i32;
     e11id: i32;
     e11textid: i32;
     e12id: i32;
     e12textid: i32;
     e13id: i32;
     e13textid: i32;
     e14id: i32;
     e14textid: i32;
     e15id: i32;
     e15textid: i32;
     e77id: i32;
     e77textid: i32;
     e80id: i32;
     e80textid: i32;
     e81id: i32;
     e81textid: i32;
     e82id: i32;
     e82textid: i32;
     e83id: i32;
     e83textid: i32;
     e84id: i32;
     e84textid: i32;
     e85id: i32;
     e85textid: i32;
     e86id: i32;
     e86textid: i32;
     e87id: i32;
     e87textid: i32;
     ListClass BasicListObj;
     BasicListId: i32;
     BBasicSpriteId: i32;
     BBasicSpriteId2: i32;
     bbasicspriteid3: i32;
     BBasicPicId: i32;
     BPreHexPicId: i32;
     BChangePreHexPicId: i32;
     BChangeBasicSpriteId: i32;
     BChangeBasicSpriteId2: i32;
     Bchangebasicspriteid3: i32;
     BChangeBasicPicId: i32;
     BPlotLastId: i32;
     BPlotLastTextId: i32;
     BRandomId: i32;
     BRandomTextId: i32;
     BAddBasicId: i32;
     BAddBasicTextId: i32;
     BRemoveBasicId: i32;
     BRemoveBasicTextId: i32;
     BDrawId: i32;
     BDrawTextId: i32;
     ListClass MoveListObj;
     MoveListId: i32;
     BMoveCostId: i32;
     BChangeMoveCostId: i32;
     BBuildGroundId: i32;
     BBuildGroundTextId: i32;
     BIsSeaId: i32;
     BIsSeaTextId: i32;
     c1id: i32;
     c1textid: i32;
     c2id: i32;
     c2textid: i32;
     c3id: i32;
     c3textid: i32;
     c4id: i32;
     c4textid: i32;
     c5id: i32;
     c5textid: i32;
     c6id: i32;
     c6textid: i32;
     ListClass SpecialListObj;
     SpecialListId: i32;
     BSpecialSpriteId: i32;
     BChangeSpecialSpriteId: i32;
     CombatListId: i32;
     ListClass CombatListObj;
     b19id: i32;
     b19textid: i32;
     b20id: i32;
     b20textid: i32;
     b18id: i32;
     b18textid: i32;
     b21id: i32;
     b21textid: i32;
     killoverrideId: i32;
     addoverrideId: i32;
     addoverrideID2: i32;
     killoverrideId2: i32;
     zoverrideId: i32;
     zoverridetextId: i32;
     ListClass SpecialList2Obj;
     ListClass specialList3Obj;
     SpecialList2Id: i32;
     speciallist3id: i32;
     i1id: i32;
     i2id: i32;
     i3id: i32;
     i1change: i32;
     i2change: i32;
     i3change: i32;
     LTNr: i32;
     TabSheetNr: i32;
     DetailNr: i32;
     Detailnr2: i32;
     OverIsTopId: i32;
     OverIsTopTextId: i32;
     ss: String;

    pub LandscapeTypeWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Landscape Types")
    {
      this.LTNr = -1;
      this.TabSheetNr = -1;
      this.DetailNr = -1;
      this.Detailnr2 = -1;
      this.MakeLTListGUI(-1);
    }

    pub fn DoRefresh() => this.MakeLandscapeTypeItemGUI();

     void MakeLTListGUI(tltnr: i32)
    {
      if (this.game.Data.LandscapeTypeCounter > -1)
      {
        this.LTListObj = ListClass::new();
        let mut landscapeTypeCounter: i32 =  this.game.Data.LandscapeTypeCounter;
        for (let mut index: i32 =  0; index <= landscapeTypeCounter; index += 1)
          this.LTListObj.add(Strings.Trim(Conversion.Str( index)) + ") " + this.game.Data.LandscapeTypeObj[index].Name, index);
        if (this.LTListId > 0)
          this.RemoveSubPart(this.LTListId);
        ListClass ltListObj = this.LTListObj;
        let mut tlistselect: i32 =  tltnr;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(ltListObj, 9, 200, tlistselect, game, tHeader: "LandscapeTypes", tbackbitmap: ( local1), bbx: 10, bby: 50, overruleFont: ( local2));
        this.LTListId = this.AddSubPart( tsubpart, 10, 50, 200, 192, 0);
        this.LTNr = tltnr;
        this.MakeLandscapeTypeItemGUI();
      }
      else
      {
        if (this.LTListId > 0)
          this.RemoveSubPart(this.LTListId);
        this.LTNr = -1;
        this.MakeLandscapeTypeItemGUI();
      }
      if (this.BAddLTId > 0)
        this.RemoveSubPart(this.BAddLTId);
      if (this.BAddLTTextId > 0)
        this.RemoveSubPart(this.BAddLTTextId);
      if (this.BCloneLTId > 0)
        this.RemoveSubPart(this.BCloneLTId);
      if (this.BCloneLTTextId > 0)
        this.RemoveSubPart(this.BCloneLTTextId);
      SubPartClass tsubpart1;
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        this.ss = "Click to add a clone of current selection";
        tsubpart1 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.BCloneLTId = this.AddSubPart( tsubpart1, 10, 250, 32, 16, 1);
        tsubpart1 =  TextPartClass::new("Add Landscape Clone", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.BCloneLTTextId = this.AddSubPart( tsubpart1, 50, 249, 300, 20, 0);
        this.ss = "Click to add a completly new landscapetype";
        tsubpart1 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.BAddLTId = this.AddSubPart( tsubpart1, 10, 270, 32, 16, 1);
        tsubpart1 =  TextPartClass::new("Add LandscapeType", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.BAddLTTextId = this.AddSubPart( tsubpart1, 50, 269, 300, 20, 0);
      }
      if (this.e7id > 0)
        this.RemoveSubPart(this.e7id);
      if (this.e7textid > 0)
        this.RemoveSubPart(this.e7textid);
      if (this.e7bid > 0)
        this.RemoveSubPart(this.e7bid);
      if (this.e7btextid > 0)
        this.RemoveSubPart(this.e7btextid);
      this.ss = "";
      this.ss = "EXPERT USE ONLY. Is a special function that can deconstruct files with all 64 sprites drawn onto in to seperate files.";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.e7id = this.AddSubPart( tsubpart1, 10, 310, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Deconstr.sheet", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: this.ss);
      this.e7textid = this.AddSubPart( tsubpart1, 50, 309, 150, 20, 0);
      this.ss = "EXPERT USE ONLY. Is a special function that can construct files with all 64 seperate 64 sprites assembled into a fred tilesheet";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.e7bid = this.AddSubPart( tsubpart1, 240, 310, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Constr.sheet", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: this.ss);
      this.e7btextid = this.AddSubPart( tsubpart1, 280, 309, 150, 20, 0);
      this.ss = "EXPERT USE ONLY. Is a special function that loads all graphics files in memory and redraws and save them in 32bppPArgb pixelformat.";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.e77id = this.AddSubPart( tsubpart1, 240, 330, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Harmonize PixForm", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: this.ss);
      this.e77textid = this.AddSubPart( tsubpart1, 280, 329, 150, 20, 0);
      this.ss = "EXPERT USE ONLY. Is a specialized function that can remove pixels outside the hex area.";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.e14id = this.AddSubPart( tsubpart1, 10, 330, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Remove Edges", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.e14textid = this.AddSubPart( tsubpart1, 50, 329, 200, 20, 0);
    }

     void MakeLandscapeTypeItemGUI()
    {
      if (this.BNameId > 0)
        this.RemoveSubPart(this.BNameId);
      if (this.BNameTextId > 0)
        this.RemoveSubPart(this.BNameTextId);
      if (this.BSLId > 0)
        this.RemoveSubPart(this.BSLId);
      if (this.BSLTextId > 0)
        this.RemoveSubPart(this.BSLTextId);
      if (this.BSL2Id > 0)
        this.RemoveSubPart(this.BSL2Id);
      if (this.BSL2TextId > 0)
        this.RemoveSubPart(this.BSL2TextId);
      if (this.BRemoveLtId > 0)
        this.RemoveSubPart(this.BRemoveLtId);
      if (this.BRemoveLtTextId > 0)
        this.RemoveSubPart(this.BRemoveLtTextId);
      if (this.OptionsListId > 0)
        this.RemoveSubPart(this.OptionsListId);
      if (this.BBuildRoadId > 0)
        this.RemoveSubPart(this.BBuildRoadId);
      if (this.BBuildRoadTextId > 0)
        this.RemoveSubPart(this.BBuildRoadTextId);
      if (this.e1id > 0)
        this.RemoveSubPart(this.e1id);
      if (this.e1textid > 0)
        this.RemoveSubPart(this.e1textid);
      if (this.e2id > 0)
        this.RemoveSubPart(this.e2id);
      if (this.e2textid > 0)
        this.RemoveSubPart(this.e2textid);
      if (this.e3id > 0)
        this.RemoveSubPart(this.e3id);
      if (this.e3textid > 0)
        this.RemoveSubPart(this.e3textid);
      if (this.c4id > 0)
        this.RemoveSubPart(this.c4id);
      if (this.c4textid > 0)
        this.RemoveSubPart(this.c4textid);
      if (this.c5id > 0)
        this.RemoveSubPart(this.c5id);
      if (this.c5textid > 0)
        this.RemoveSubPart(this.c5textid);
      if (this.c6id > 0)
        this.RemoveSubPart(this.c6id);
      if (this.c6textid > 0)
        this.RemoveSubPart(this.c6textid);
      if (this.e4id > 0)
        this.RemoveSubPart(this.e4id);
      if (this.e4textid > 0)
        this.RemoveSubPart(this.e4textid);
      if (this.e5id > 0)
        this.RemoveSubPart(this.e5id);
      if (this.e5textid > 0)
        this.RemoveSubPart(this.e5textid);
      if (this.e6id > 0)
        this.RemoveSubPart(this.e6id);
      if (this.e6textid > 0)
        this.RemoveSubPart(this.e6textid);
      if (this.e8id > 0)
        this.RemoveSubPart(this.e8id);
      if (this.e8textid > 0)
        this.RemoveSubPart(this.e8textid);
      if (this.e9id > 0)
        this.RemoveSubPart(this.e9id);
      if (this.e9textid > 0)
        this.RemoveSubPart(this.e9textid);
      if (this.e80id > 0)
        this.RemoveSubPart(this.e80id);
      if (this.e80textid > 0)
        this.RemoveSubPart(this.e80textid);
      if (this.e81id > 0)
        this.RemoveSubPart(this.e81id);
      if (this.e81textid > 0)
        this.RemoveSubPart(this.e81textid);
      if (this.e82id > 0)
        this.RemoveSubPart(this.e82id);
      if (this.e82textid > 0)
        this.RemoveSubPart(this.e82textid);
      if (this.e83id > 0)
        this.RemoveSubPart(this.e83id);
      if (this.e83textid > 0)
        this.RemoveSubPart(this.e83textid);
      if (this.e10id > 0)
        this.RemoveSubPart(this.e10id);
      if (this.e10textid > 0)
        this.RemoveSubPart(this.e10textid);
      if (this.e11id > 0)
        this.RemoveSubPart(this.e11id);
      if (this.e11textid > 0)
        this.RemoveSubPart(this.e11textid);
      if (this.e12id > 0)
        this.RemoveSubPart(this.e12id);
      if (this.e12textid > 0)
        this.RemoveSubPart(this.e12textid);
      if (this.e13id > 0)
        this.RemoveSubPart(this.e13id);
      if (this.e13textid > 0)
        this.RemoveSubPart(this.e13textid);
      if (this.e15id > 0)
        this.RemoveSubPart(this.e15id);
      if (this.e15textid > 0)
        this.RemoveSubPart(this.e15textid);
      if (this.e84id > 0)
        this.RemoveSubPart(this.e84id);
      if (this.e84textid > 0)
        this.RemoveSubPart(this.e84textid);
      if (this.e85id > 0)
        this.RemoveSubPart(this.e85id);
      if (this.e85textid > 0)
        this.RemoveSubPart(this.e85textid);
      if (this.e86id > 0)
        this.RemoveSubPart(this.e86id);
      if (this.e86textid > 0)
        this.RemoveSubPart(this.e86textid);
      if (this.e87id > 0)
        this.RemoveSubPart(this.e87id);
      if (this.e87textid > 0)
        this.RemoveSubPart(this.e87textid);
      if (this.BPreHexPicId > 0)
        this.RemoveSubPart(this.BPreHexPicId);
      if (this.BChangePreHexPicId > 0)
        this.RemoveSubPart(this.BChangePreHexPicId);
      if (this.TxtId > 0)
        this.RemoveSubPart(this.TxtId);
      if (this.TxtStrId > 0)
        this.RemoveSubPart(this.TxtStrId);
      if (this.OverIsTopId > 0)
        this.RemoveSubPart(this.OverIsTopId);
      if (this.OverIsTopTextId > 0)
        this.RemoveSubPart(this.OverIsTopTextId);
      this.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 100, -1);
      this.FlagAll();
      if (this.LTNr > -1)
      {
        this.ss = "Click to change the name of this LandscapeType";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.BNameId = this.AddSubPart( tsubpart, 370, 50, 32, 16, 1);
        }
        let mut tsubpart1: SubPartClass =  TextPartClass::new("Name: " + this.game.Data.LandscapeTypeObj[this.LTNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.BNameTextId = this.AddSubPart( tsubpart1, 410, 49, 200, 20, 0);
        this.ss = "Click to change the description of this LandscapeType";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.TxtId = this.AddSubPart( tsubpart2, 650, 50, 32, 16, 1);
        }
        let mut tsubpart3: SubPartClass =  TextPartClass::new("Description", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.TxtStrId = this.AddSubPart( tsubpart3, 700, 49, 200, 20, 0);
        if (this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer)
        {
          this.ss = "Click to remove SpecialLayer.";
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            let mut tsubpart4: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
            this.BSLId = this.AddSubPart( tsubpart4, 370, 70, 32, 16, 1);
          }
        }
        else
        {
          this.ss = "Click to activate SpecialLayer. Which enables you to set the 64 sprites for this LT. You will be prompted where to import files from.";
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
            this.BSLId = this.AddSubPart( tsubpart5, 370, 70, 32, 16, 1);
          }
        }
        let mut tsubpart6: SubPartClass =  TextPartClass::new("x64", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 50, false, tDescript: this.ss);
        this.BSLTextId = this.AddSubPart( tsubpart6, 410, 69, 80, 20, 0);
        if (this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer6)
        {
          this.ss = "Click to remove first 6 sprite use only limit.";
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONFLAGGED, tDescript: this.ss);
            this.BSL2Id = this.AddSubPart( tsubpart7, 490, 70, 32, 16, 1);
          }
        }
        else
        {
          this.ss = "Click to add first 6 sprites use only limit";
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            let mut tsubpart8: SubPartClass =  ButtonPartClass::new(this.game.BUTTONUNFLAGGED, tDescript: this.ss);
            this.BSL2Id = this.AddSubPart( tsubpart8, 490, 70, 32, 16, 1);
          }
        }
        let mut tsubpart9: SubPartClass =  TextPartClass::new("x6", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 50, false, tDescript: this.ss);
        this.BSL2TextId = this.AddSubPart( tsubpart9, 540, 69, 80, 20, 0);
        this.ss = "Click to set AIBlock. 0=none , 1=yes.. means AI will not try to move through this lt";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          let mut tsubpart10: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e2id = this.AddSubPart( tsubpart10, 370, 130, 32, 16, 1);
        }
        let mut tsubpart11: SubPartClass =  TextPartClass::new("AIBlock: " + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].AIBlock), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e2textid = this.AddSubPart( tsubpart11, 410, 129, 200, 20, 0);
        this.ss = "Click to toggle on/off if a unit can be paradropped on a hex with this landscape";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e3id = this.AddSubPart( tsubpart11, 610, 130, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("CanParadrop: " + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].CanParadrop), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e3textid = this.AddSubPart( tsubpart11, 650, 129, 200, 20, 0);
        this.ss = "Click to toggle on/off if a unit can be amphibiously unloaded on a hex with this landscape";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e5id = this.AddSubPart( tsubpart11, 610, 90, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("CanAmph: " + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].CanAmph), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e5textid = this.AddSubPart( tsubpart11, 650, 89, 200, 20, 0);
        this.ss = "Click to set for: Color landscapetype on minimap. press cancel for no color.";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e6id = this.AddSubPart( tsubpart11, 850, 150, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("Color: " + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].Red) + "," + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].Green) + "," + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].Blue), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e6textid = this.AddSubPart( tsubpart11, 890, 149, 200, 20, 0);
        if (this.game.Data.Product >= 6)
        {
          this.ss = "Click to set the number of Obstruct Points for the Landscape. Only used with Advanced Recon Rules.";
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
            this.e86id = this.AddSubPart( tsubpart11, 850, 130, 32, 16, 1);
          }
          tsubpart11 =  TextPartClass::new("Obstruct: " + this.game.Data.LandscapeTypeObj[this.LTNr].Obstruction.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.e86textid = this.AddSubPart( tsubpart11, 890, 129, 200, 20, 0);
        }
        this.ss = "If Interior style=false the 6/64 special sprites will be portrayd on neighbour hexes. Otherwise in own hex. Interior cannot use 6, only 64";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e8id = this.AddSubPart( tsubpart11, 850, 180, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("Interior Drawing Style = " + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].Interior), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e8textid = this.AddSubPart( tsubpart11, 880, 179, 200, 20, 0);
        this.ss = "Will use specified landscapetype 6/64 special sprites as exterior transitions. even if this landscape is set on Interior itself. ";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e9id = this.AddSubPart( tsubpart11, 850, 200, 32, 16, 1);
        }
        if (this.game.Data.LandscapeTypeObj[this.LTNr].ExtraExterior == -1)
        {
          tsubpart11 =  TextPartClass::new("ExtraExterior = -1", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.e9textid = this.AddSubPart( tsubpart11, 880, 199, 200, 20, 0);
        }
        else
        {
          tsubpart11 =  TextPartClass::new("ExtraExterior = " + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].ExtraExterior) + "," + this.game.Data.LandscapeTypeObj[this.game.Data.LandscapeTypeObj[this.LTNr].ExtraExterior].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.e9textid = this.AddSubPart( tsubpart11, 880, 199, 200, 20, 0);
        }
        this.ss = "If Extra Exterior=True then the Interior transition will consider the hex with the exterior transition the same landscape";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e10id = this.AddSubPart( tsubpart11, 850, 220, 32, 16, 1);
        }
        if (this.game.Data.LandscapeTypeObj[this.LTNr].ExtraExteriorSame)
        {
          tsubpart11 =  TextPartClass::new("ExtraExteriorSame = True", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.e10textid = this.AddSubPart( tsubpart11, 880, 219, 200, 20, 0);
        }
        else
        {
          tsubpart11 =  TextPartClass::new("ExtraExteriorSame = False ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.e10textid = this.AddSubPart( tsubpart11, 880, 219, 200, 20, 0);
        }
        this.ss = "Blacked Out LT doesnt show frontiers or map edges";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e11id = this.AddSubPart( tsubpart11, 850, 240, 32, 16, 1);
        }
        if (this.game.Data.LandscapeTypeObj[this.LTNr].BlackedOut)
        {
          tsubpart11 =  TextPartClass::new("BlackedOut = True", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.e11textid = this.AddSubPart( tsubpart11, 880, 239, 200, 20, 0);
        }
        else
        {
          tsubpart11 =  TextPartClass::new("BlackedOut = False ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.e11textid = this.AddSubPart( tsubpart11, 880, 239, 200, 20, 0);
        }
        this.ss = "Use PreHex borders defined in LT X. -1=dont use.";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e12id = this.AddSubPart( tsubpart11, 850, 260, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("PreHexBorder=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].PreHexBorder), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e12textid = this.AddSubPart( tsubpart11, 880, 259, 200, 20, 0);
        this.ss = "If LT shows up in info or editor paselection: i32 windows ingame or not.";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e13id = this.AddSubPart( tsubpart11, 850, 280, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("DontShowInList=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].DontShowInList), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e13textid = this.AddSubPart( tsubpart11, 880, 279, 200, 20, 0);
        this.ss = "If NoPortReq=true it means that supply movement between land<>sea does not suffer rulevar82 penalty for having no port present.";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e15id = this.AddSubPart( tsubpart11, 850, 300, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("NoPortReq=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].NoPortReq), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e15textid = this.AddSubPart( tsubpart11, 880, 299, 200, 20, 0);
        if (this.game.Data.Product >= 7)
        {
          this.ss = "";
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
            this.e85id = this.AddSubPart( tsubpart11, 850, 320, 32, 16, 1);
          }
          tsubpart11 =  TextPartClass::new("PreHexBorderOwnZ=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].usePreHexBorderOwnZ), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.e85textid = this.AddSubPart( tsubpart11, 880, 319, 200, 20, 0);
        }
        this.ss = "Set tranparent true/false (usefull in combination with overlay map graphic)";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.c6id = this.AddSubPart( tsubpart11, 850, 340, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("Transparent=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].Transparent), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.c6textid = this.AddSubPart( tsubpart11, 880, 339, 200, 20, 0);
        this.ss = "Experimental: Use Pre Hex Texture?";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e80id = this.AddSubPart( tsubpart11, 850, 390, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("UsePreHexTexture=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexTexture), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.e80textid = this.AddSubPart( tsubpart11, 880, 389, 400, 20, 0);
        this.ss = this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureFileName;
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e81id = this.AddSubPart( tsubpart11, 850, 370, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("PreHexTextureFile=" + this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureFileName, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.e81textid = this.AddSubPart( tsubpart11, 880, 369, 400, 20, 0);
        this.ss = Conversions.ToString(this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexBorderTexture);
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e82id = this.AddSubPart( tsubpart11, 850, 410, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("UsePreHexBordersTex=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexBorderTexture), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.e82textid = this.AddSubPart( tsubpart11, 880, 409, 400, 20, 0);
        this.ss = "";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e83id = this.AddSubPart( tsubpart11, 850, 430, 32, 16, 1);
        }
        tsubpart11 =  TextPartClass::new("UsePreHexTextureAndRegularPreHex=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexTextureAndRegularPreHex), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 600, 20, false, tDescript: this.ss);
        this.e83textid = this.AddSubPart( tsubpart11, 880, 429, 600, 20, 0);
        if (this.game.Data.Product >= 6)
        {
          this.ss = "";
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
            this.e87id = this.AddSubPart( tsubpart11, 850, 470, 32, 16, 1);
          }
          tsubpart11 =  TextPartClass::new("FuzzyOwnerAssured=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].FuzzyOwnerAssured), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 600, 20, false, tDescript: this.ss);
          this.e87textid = this.AddSubPart( tsubpart11, 880, 469, 600, 20, 0);
        }
        if (this.game.Data.LandscapeTypeCounter > 0)
        {
          this.ss = "Click to remove this landscapetype";
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            tsubpart11 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
            this.BRemoveLtId = this.AddSubPart( tsubpart11, 10, 290, 32, 16, 1);
          }
          if (Strings.Len(this.game.Data.MasterFile) == 0)
          {
            tsubpart11 =  TextPartClass::new("Remove", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
            this.BRemoveLtTextId = this.AddSubPart( tsubpart11, 50, 289, 200, 20, 0);
          }
        }
        this.OptionsListObj = ListClass::new();
        this.OptionsListObj.add("Sprites", 0);
        this.OptionsListObj.add("Movecost + Stats", 1);
        if (this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer | this.game.Data.LandscapeTypeObj[this.LTNr].UseSheet)
        {
          this.OptionsListObj.add("64 Special Sprites", 2);
          this.OptionsListObj.add("Graphic overrides", 3);
        }
        else
        {
          this.OptionsListObj.add("64 Special Sprites", 2);
          this.OptionsListObj.add("Graphic overrides", 3);
        }
        this.OptionsListObj.add("Entrench per Unitgroup", 4);
        this.OptionsListObj.add("Landscape Layers", 5);
        ListClass optionsListObj = this.OptionsListObj;
        let mut tabSheetNr: i32 =  this.TabSheetNr;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart11 =  new ListSubPartClass(optionsListObj, 4, 300, tabSheetNr, game, tHeader: "Property Sheets", tbackbitmap: ( local1), bbx: 370, bby: 150, overruleFont: ( local2));
        this.OptionsListId = this.AddSubPart( tsubpart11, 370, 150, 300, 112, 0);
        this.ss = "Click to change the prehex sprite. current file = " + this.game.Data.LandscapeTypeObj[this.LTNr].PreHexPicFileName;
        tsubpart11 =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].PreHexPicID, tDescript: this.ss, tResizeX: 64, tresizeY: 48);
        this.BPreHexPicId = this.AddSubPart( tsubpart11, 410, 270, 64, 48, 0);
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart11 =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
          this.BChangePreHexPicId = this.AddSubPart( tsubpart11, 370, 270, 32, 16, 1);
        }
      }
      this.maketabsheet();
    }

     void maketabsheet()
    {
      if (this.BasicListId > 0)
        this.RemoveSubPart(this.BasicListId);
      if (this.BBasicSpriteId > 0)
        this.RemoveSubPart(this.BBasicSpriteId);
      if (this.BChangeBasicSpriteId > 0)
        this.RemoveSubPart(this.BChangeBasicSpriteId);
      if (this.BBasicSpriteId2 > 0)
        this.RemoveSubPart(this.BBasicSpriteId2);
      if (this.BChangeBasicSpriteId2 > 0)
        this.RemoveSubPart(this.BChangeBasicSpriteId2);
      if (this.bbasicspriteid3 > 0)
        this.RemoveSubPart(this.bbasicspriteid3);
      if (this.Bchangebasicspriteid3 > 0)
        this.RemoveSubPart(this.Bchangebasicspriteid3);
      if (this.BBasicPicId > 0)
        this.RemoveSubPart(this.BBasicPicId);
      if (this.BChangeBasicPicId > 0)
        this.RemoveSubPart(this.BChangeBasicPicId);
      if (this.SpecialListId > 0)
        this.RemoveSubPart(this.SpecialListId);
      if (this.BSpecialSpriteId > 0)
        this.RemoveSubPart(this.BSpecialSpriteId);
      if (this.BChangeSpecialSpriteId > 0)
        this.RemoveSubPart(this.BChangeSpecialSpriteId);
      if (this.BAddBasicId > 0)
        this.RemoveSubPart(this.BAddBasicId);
      if (this.BAddBasicTextId > 0)
        this.RemoveSubPart(this.BAddBasicTextId);
      if (this.BRemoveBasicId > 0)
        this.RemoveSubPart(this.BRemoveBasicId);
      if (this.BRemoveBasicTextId > 0)
        this.RemoveSubPart(this.BRemoveBasicTextId);
      if (this.BDrawId > 0)
        this.RemoveSubPart(this.BDrawId);
      if (this.BDrawTextId > 0)
        this.RemoveSubPart(this.BDrawTextId);
      if (this.i1id > 0)
        this.RemoveSubPart(this.i1id);
      if (this.i1change > 0)
        this.RemoveSubPart(this.i1change);
      if (this.i2id > 0)
        this.RemoveSubPart(this.i2id);
      if (this.i2change > 0)
        this.RemoveSubPart(this.i2change);
      if (this.i3id > 0)
        this.RemoveSubPart(this.i3id);
      if (this.i3change > 0)
        this.RemoveSubPart(this.i3change);
      if (this.MoveListId > 0)
        this.RemoveSubPart(this.MoveListId);
      if (this.BMoveCostId > 0)
        this.RemoveSubPart(this.BMoveCostId);
      if (this.BChangeMoveCostId > 0)
        this.RemoveSubPart(this.BChangeMoveCostId);
      if (this.BBuildGroundId > 0)
        this.RemoveSubPart(this.BBuildGroundId);
      if (this.BBuildGroundTextId > 0)
        this.RemoveSubPart(this.BBuildGroundTextId);
      if (this.BPlotLastId > 0)
        this.RemoveSubPart(this.BPlotLastId);
      if (this.BPlotLastTextId > 0)
        this.RemoveSubPart(this.BPlotLastTextId);
      if (this.BIsSeaId > 0)
        this.RemoveSubPart(this.BIsSeaId);
      if (this.BIsSeaTextId > 0)
        this.RemoveSubPart(this.BIsSeaTextId);
      if (this.b18id > 0)
        this.RemoveSubPart(this.b18id);
      if (this.b18textid > 0)
        this.RemoveSubPart(this.b18textid);
      if (this.b19id > 0)
        this.RemoveSubPart(this.b19id);
      if (this.b19textid > 0)
        this.RemoveSubPart(this.b19textid);
      if (this.b20id > 0)
        this.RemoveSubPart(this.b20id);
      if (this.b20textid > 0)
        this.RemoveSubPart(this.b20textid);
      if (this.CombatListId > 0)
        this.RemoveSubPart(this.CombatListId);
      if (this.c1id > 0)
        this.RemoveSubPart(this.c1id);
      if (this.c1textid > 0)
        this.RemoveSubPart(this.c1textid);
      if (this.c2id > 0)
        this.RemoveSubPart(this.c2id);
      if (this.c2textid > 0)
        this.RemoveSubPart(this.c2textid);
      if (this.c3id > 0)
        this.RemoveSubPart(this.c3id);
      if (this.c3textid > 0)
        this.RemoveSubPart(this.c3textid);
      if (this.killoverrideId > 0)
        this.RemoveSubPart(this.killoverrideId);
      if (this.addoverrideId > 0)
        this.RemoveSubPart(this.addoverrideId);
      if (this.killoverrideId2 > 0)
        this.RemoveSubPart(this.killoverrideId2);
      if (this.addoverrideID2 > 0)
        this.RemoveSubPart(this.addoverrideID2);
      if (this.zoverrideId > 0)
        this.RemoveSubPart(this.zoverrideId);
      if (this.zoverridetextId > 0)
        this.RemoveSubPart(this.zoverridetextId);
      if (this.SpecialList2Id > 0)
        this.RemoveSubPart(this.SpecialList2Id);
      if (this.speciallist3id > 0)
        this.RemoveSubPart(this.speciallist3id);
      if (this.BRandomId > 0)
        this.RemoveSubPart(this.BRandomId);
      if (this.BRandomTextId > 0)
        this.RemoveSubPart(this.BRandomTextId);
      if (this.OverIsTopId > 0)
        this.RemoveSubPart(this.OverIsTopId);
      if (this.OverIsTopTextId > 0)
        this.RemoveSubPart(this.OverIsTopTextId);
      if (!(this.LTNr > -1 & this.TabSheetNr > -1))
        return;
      if (this.TabSheetNr == 0)
        this.maketabsheetnr0();
      if (this.TabSheetNr == 1)
        this.maketabsheetnr1();
      if (this.TabSheetNr == 2)
        this.maketabsheetnr2();
      if (this.TabSheetNr == 3)
        this.maketabsheetnr3();
      if (this.TabSheetNr == 4)
        this.maketabsheetnr4();
      if (this.TabSheetNr != 5)
        return;
      this.maketabsheetnr5();
    }

     void maketabsheetnr5()
    {
      if (this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteCounter <= -1)
        return;
      this.BasicListObj = ListClass::new();
      let mut basicSpriteCounter: i32 =  this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteCounter;
      for (let mut tdata: i32 =  0; tdata <= basicSpriteCounter; tdata += 1)
        this.BasicListObj.add(this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteFileName[tdata], tdata);
      ListClass basicListObj = this.BasicListObj;
      let mut detailNr: i32 =  this.DetailNr;
      let mut game: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(basicListObj, 10, 300, detailNr, game, tHeader: "Sprites", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
      this.BasicListId = this.AddSubPart( tsubpart, 10, 350, 300, 208, 0);
      if (this.DetailNr > this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteCounter)
        this.DetailNr = -1;
      if (this.DetailNr <= -1)
        return;
      this.maketabsheetnr5b();
    }

     void maketabsheetnr5b()
    {
      if (this.i1id > 0)
        this.RemoveSubPart(this.i1id);
      if (this.i1change > 0)
        this.RemoveSubPart(this.i1change);
      if (this.i2id > 0)
        this.RemoveSubPart(this.i2id);
      if (this.i2change > 0)
        this.RemoveSubPart(this.i2change);
      if (this.i3id > 0)
        this.RemoveSubPart(this.i3id);
      if (this.i3change > 0)
        this.RemoveSubPart(this.i3change);
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].SidewaysSPriteID1[this.DetailNr], tDescript: "1st (sky layer)", tResizeX: 100, tresizeY: 41);
      this.i1id = this.AddSubPart( tsubpart1, 410, 350, 140, 80, 1);
      let mut tsubpart2: SubPartClass =  ButtonPartClass::new(BitmapStore.GetBitmap(this.game.BUTTONBLUE), "1st (sky layer)");
      this.i1change = this.AddSubPart( tsubpart2, 410, 450, 32, 16, 1);
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].SidewaysSPriteID2[this.DetailNr], tDescript: "2nd (back layer)", tResizeX: 100, tresizeY: 41);
      this.i2id = this.AddSubPart( tsubpart3, 610, 350, 140, 80, 1);
      let mut tsubpart4: SubPartClass =  ButtonPartClass::new(BitmapStore.GetBitmap(this.game.BUTTONBLUE), "2nd (back layer)");
      this.i2change = this.AddSubPart( tsubpart4, 610, 450, 32, 16, 1);
      let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].SidewaysSPriteID3[this.DetailNr], tDescript: "3rd (front layer)", tResizeX: 100, tresizeY: 41);
      this.i3id = this.AddSubPart( tsubpart5, 810, 350, 140, 80, 1);
      let mut tsubpart6: SubPartClass =  ButtonPartClass::new(BitmapStore.GetBitmap(this.game.BUTTONBLUE), "3rd (front layer)");
      this.i3change = this.AddSubPart( tsubpart6, 810, 450, 32, 16, 1);
    }

     void maketabsheetnr0()
    {
      if (this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteCounter > -1)
      {
        this.BasicListObj = ListClass::new();
        let mut basicSpriteCounter: i32 =  this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteCounter;
        for (let mut tdata: i32 =  0; tdata <= basicSpriteCounter; tdata += 1)
          this.BasicListObj.add(this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteFileName[tdata], tdata);
        ListClass basicListObj = this.BasicListObj;
        let mut detailNr: i32 =  this.DetailNr;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(basicListObj, 10, 300, detailNr, game, tHeader: "Sprites", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
        this.BasicListId = this.AddSubPart( tsubpart, 10, 350, 300, 208, 0);
        if (this.DetailNr > this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteCounter)
          this.DetailNr = -1;
        if (this.DetailNr > -1)
          this.maketabsheetnr0b();
      }
      this.ss = "Click to add a sprite to selected landscapetype";
      SubPartClass tsubpart1;
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart1 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.BAddBasicId = this.AddSubPart( tsubpart1, 10, 580, 32, 16, 1);
      }
      if (Strings.Len(this.game.Data.MasterFile) != 0)
        return;
      tsubpart1 =  TextPartClass::new("Add Sprite", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.BAddBasicTextId = this.AddSubPart( tsubpart1, 50, 579, 200, 20, 0);
    }

     void maketabsheetnr0b()
    {
      if (this.BBasicSpriteId > 0)
        this.RemoveSubPart(this.BBasicSpriteId);
      if (this.BChangeBasicSpriteId > 0)
        this.RemoveSubPart(this.BChangeBasicSpriteId);
      this.ss = "Click to change the sprite";
      let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteID[this.DetailNr], tDescript: this.ss, tResizeX: 64, tresizeY: 48);
      this.BBasicSpriteId = this.AddSubPart( tsubpart, 400, 350, 64, 48, 0);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
        this.BChangeBasicSpriteId = this.AddSubPart( tsubpart, 400, 410, 32, 16, 1);
      }
      this.ss = "Click to change the optional graphic that is optionally drawn over the sprite at end of drawing";
      if (this.BBasicSpriteId2 > 0)
        this.RemoveSubPart(this.BBasicSpriteId2);
      if (this.BChangeBasicSpriteId2 > 0)
        this.RemoveSubPart(this.BChangeBasicSpriteId2);
      tsubpart =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteID2[this.DetailNr], tDescript: this.ss, tResizeX: 64, tresizeY: 48);
      this.BBasicSpriteId2 = this.AddSubPart( tsubpart, 470, 350, 64, 48, 0);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
        this.BChangeBasicSpriteId2 = this.AddSubPart( tsubpart, 470, 410, 32, 16, 1);
      }
      this.ss = "Click to change the optional OverIsTop graphics optionally drawn over the hex above the landscape";
      if (this.bbasicspriteid3 > 0)
        this.RemoveSubPart(this.bbasicspriteid3);
      if (this.Bchangebasicspriteid3 > 0)
        this.RemoveSubPart(this.Bchangebasicspriteid3);
      tsubpart =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteID3[this.DetailNr], tDescript: this.ss, tResizeX: 64, tresizeY: 48);
      this.bbasicspriteid3 = this.AddSubPart( tsubpart, 540, 350, 64, 48, 0);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
        this.Bchangebasicspriteid3 = this.AddSubPart( tsubpart, 540, 410, 32, 16, 1);
      }
      this.ss = "Click to change the artistic picture representing this sprite for this landscapetype";
      if (this.BBasicPicId > 0)
        this.RemoveSubPart(this.BBasicPicId);
      if (this.BChangeBasicPicId > 0)
        this.RemoveSubPart(this.BChangeBasicPicId);
      tsubpart =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].BasicPicID[this.DetailNr], tDescript: this.ss, tResizeX: 363, tresizeY: 150);
      this.BBasicPicId = this.AddSubPart( tsubpart, 400, 450, 363, 150, 0);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
        this.BChangeBasicPicId = this.AddSubPart( tsubpart, 780, 500, 32, 16, 1);
      }
      this.ss = "Click to toggle on/off the optional drawing over of the optional graphic";
      if (this.BPlotLastId > 0)
        this.RemoveSubPart(this.BPlotLastId);
      if (this.BPlotLastTextId > 0)
        this.RemoveSubPart(this.BPlotLastTextId);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BPlotLastId = this.AddSubPart( tsubpart, 640, 350, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("PlotLast:" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].PlotLast[this.DetailNr]), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 110, 20, false, tDescript: this.ss);
      this.BPlotLastTextId = this.AddSubPart( tsubpart, 690, 349, 110, 20, 0);
      if (this.game.Data.Product >= 7)
      {
        this.ss = "Click to toggle on/off the optional drawing before the River";
        if (this.e84id > 0)
          this.RemoveSubPart(this.e84id);
        if (this.e84textid > 0)
          this.RemoveSubPart(this.e84textid);
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.e84id = this.AddSubPart( tsubpart, 640, 370, 32, 16, 1);
        }
        tsubpart =  TextPartClass::new("BeforeRiv:" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].PlotBeforeRiver[this.DetailNr]), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 110, 20, false, tDescript: this.ss);
        this.e84textid = this.AddSubPart( tsubpart, 690, 369, 110, 20, 0);
      }
      this.ss = "OverIsTop graphics is used as overdraw on the hex above it = true/false. If overistop=true 64 border sprites dont work.";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.OverIsTopId = this.AddSubPart( tsubpart, 640, 330, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("OverIsTop:" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].OverIsTop[this.DetailNr]), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 110, 20, false, tDescript: this.ss);
      this.OverIsTopTextId = this.AddSubPart( tsubpart, 690, 329, 110, 20, 0);
      if (this.BRemoveBasicId > 0)
        this.RemoveSubPart(this.BRemoveBasicId);
      if (this.BRemoveBasicTextId > 0)
        this.RemoveSubPart(this.BRemoveBasicTextId);
      if (this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteCounter > 0)
      {
        this.ss = "Click to remove the selected sprite";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
          this.BRemoveBasicId = this.AddSubPart( tsubpart, 10, 600, 32, 16, 1);
        }
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart =  TextPartClass::new("Remove this Basic Sprite", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
          this.BRemoveBasicTextId = this.AddSubPart( tsubpart, 50, 599, 200, 20, 0);
        }
      }
      if (this.BDrawId > 0)
        this.RemoveSubPart(this.BDrawId);
      if (this.BDrawTextId > 0)
        this.RemoveSubPart(this.BDrawTextId);
      this.ss = "Click to select this sprite for drawing on the map";
      tsubpart =  ButtonPartClass::new(this.game.BUTTONDRAW, tDescript: this.ss);
      this.BDrawId = this.AddSubPart( tsubpart, 10, 620, 32, 16, 1);
      tsubpart =  TextPartClass::new("Select as pencil", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.BDrawTextId = this.AddSubPart( tsubpart, 50, 619, 200, 20, 0);
    }

     void maketabsheetnr1()
    {
      this.MoveListObj = ListClass::new();
      let mut index: i32 =  0;
      do
      {
        this.MoveListObj.add(Conversion.Str( index) + ") " + this.game.Data.TempString[index] + " = " + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].MoveCost[index]), index);
        index += 1;
      }
      while (index <= 99);
      let mut num: i32 =   Math.Round( Math.Max(0, this.game.ScreenHeight - 800) / 16.0);
      ListClass moveListObj = this.MoveListObj;
      let mut tlistsize: i32 =  10 + num;
      let mut detailNr: i32 =  this.DetailNr;
      let mut game: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(moveListObj, tlistsize, 300, detailNr, game, tHeader: "Movecost for MoveTypes", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
      this.MoveListId = this.AddSubPart( tsubpart1, 10, 350, 300, (13 + num) * 16, 0);
      if (this.DetailNr > 99)
        this.DetailNr = -1;
      if (this.DetailNr > -1)
        this.maketabsheetnr1b();
      this.ss = "Click to set the Buildground Type";
      SubPartClass tsubpart2;
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BBuildGroundId = this.AddSubPart( tsubpart2, 410, 350, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("BuildGround =" + this.game.Data.TempString[100 + this.game.Data.LandscapeTypeObj[this.LTNr].BuildGround], Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.BBuildGroundTextId = this.AddSubPart( tsubpart2, 450, 349, 200, 20, 0);
      this.ss = "Click to set if landscapetype is 'sea' or not.";
      if (this.game.Data.LandscapeTypeObj[this.LTNr].IsSea)
      {
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart2 =  ButtonPartClass::new(this.game.BUTTONFLAGGED, tDescript: this.ss);
          this.BIsSeaId = this.AddSubPart( tsubpart2, 410, 380, 32, 16, 1);
        }
      }
      else if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(this.game.BUTTONUNFLAGGED, tDescript: this.ss);
        this.BIsSeaId = this.AddSubPart( tsubpart2, 410, 380, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("Is Sea", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 50, false, tDescript: this.ss);
      this.BIsSeaTextId = this.AddSubPart( tsubpart2, 450, 379, 200, 20, 0);
      this.ss = "Set the hide points a unit in this landscapetype gets.";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        tsubpart2 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.c3id = this.AddSubPart( tsubpart2, 410, 440, 32, 16, 1);
      }
      tsubpart2 =  TextPartClass::new("HidePTs =" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].HidePts), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.c3textid = this.AddSubPart( tsubpart2, 450, 439, 200, 20, 0);
    }

     void maketabsheetnr1b()
    {
      txt: String = "Move Cost = " + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].MoveCost[this.DetailNr]);
      if (this.BMoveCostId > 0)
        this.RemoveSubPart(this.BMoveCostId);
      if (this.BChangeMoveCostId > 0)
        this.RemoveSubPart(this.BChangeMoveCostId);
      this.ss = "Click to set new movecost for this movetype for this landscape type in Action Points";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BChangeMoveCostId = this.AddSubPart( tsubpart, 400, 490, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new(txt, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.BMoveCostId = this.AddSubPart( tsubpart1, 450, 490, 200, 20, 0);
    }

     void maketabsheetnr2()
    {
      if (!this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer)
        return;
      if (!this.game.Data.LandscapeTypeObj[this.LTNr].UseSheet)
      {
        if (this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexBorderTexture)
          return;
        this.SpecialListObj = ListClass::new();
        let mut tdata: i32 =  0;
        do
        {
          this.SpecialListObj.add(this.game.Data.LandscapeTypeObj[this.LTNr].LayerSpriteFileName[tdata + 1], tdata);
          tdata += 1;
        }
        while (tdata <= 63);
        ListClass specialListObj = this.SpecialListObj;
        let mut detailNr: i32 =  this.DetailNr;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(specialListObj, 15, 300, detailNr, game, tHeader: "64 Special Sprites", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
        this.SpecialListId = this.AddSubPart( tsubpart, 10, 350, 300, 288, 0);
        if (this.DetailNr > 63)
          this.DetailNr = -1;
        if (this.DetailNr <= -1)
          return;
        this.maketabsheetnr2b();
      }
      else if (this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexBorderTexture | this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexTexture)
      {
        this.ss = "the fred sheet you are currently using. filename = " + this.game.Data.LandscapeTypeObj[this.LTNr].SheetFileName;
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BIsSeaTextId = this.AddSubPart( tsubpart, 10, 350, 32, 16, 1);
      }
      else
      {
        this.ss = "the fred sheet you are currently using. filename = " + this.game.Data.LandscapeTypeObj[this.LTNr].SheetFileName;
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID, tDescript: this.ss);
        this.BIsSeaTextId = this.AddSubPart( tsubpart, 10, 350, BitmapStore.GetWidth(this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID), BitmapStore.Getheight(this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID), 0);
      }
    }

     void maketabsheetnr2b()
    {
      if (this.BSpecialSpriteId > 0)
        this.RemoveSubPart(this.BSpecialSpriteId);
      if (this.BChangeSpecialSpriteId > 0)
        this.RemoveSubPart(this.BChangeSpecialSpriteId);
      this.ss = "Click to replace a single selected special sprite. Advice: disable specialsprites, enable again and select dir to input all 64 from.";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.Data.LandscapeTypeObj[this.LTNr].LayerSpriteID[this.DetailNr + 1], tDescript: this.ss);
      this.BSpecialSpriteId = this.AddSubPart( tsubpart1, 400, 350, 64, 48, 0);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
        this.BChangeSpecialSpriteId = this.AddSubPart( tsubpart2, 400, 410, 32, 16, 1);
      }
      this.ss = "Click to set for: Color landscapetype on minimap. press cancel for no color.";
    }

    pub fn maketabsheetnr3()
    {
      if (this.killoverrideId > 0)
        this.RemoveSubPart(this.killoverrideId);
      if (this.addoverrideId > 0)
        this.RemoveSubPart(this.addoverrideId);
      if (this.killoverrideId2 > 0)
        this.RemoveSubPart(this.killoverrideId2);
      if (this.addoverrideID2 > 0)
        this.RemoveSubPart(this.addoverrideID2);
      if (this.zoverrideId > 0)
        this.RemoveSubPart(this.zoverrideId);
      if (this.zoverridetextId > 0)
        this.RemoveSubPart(this.zoverridetextId);
      if (this.SpecialList2Id > 0)
        this.RemoveSubPart(this.SpecialList2Id);
      if (this.speciallist3id > 0)
        this.RemoveSubPart(this.speciallist3id);
      this.SpecialList2Obj = ListClass::new();
      if (this.game.Data.LandscapeTypeObj[this.LTNr].OverridesCount > -1)
      {
        let mut overridesCount: i32 =  this.game.Data.LandscapeTypeObj[this.LTNr].OverridesCount;
        for (let mut tdata: i32 =  0; tdata <= overridesCount; tdata += 1)
          this.SpecialList2Obj.add(Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].OverridesType[tdata]) + ") " + this.game.Data.LandscapeTypeObj[this.game.Data.LandscapeTypeObj[this.LTNr].OverridesType[tdata]].Name, tdata);
        if (this.DetailNr > this.game.Data.LandscapeTypeObj[this.LTNr].OverridesCount)
          this.DetailNr = this.game.Data.LandscapeTypeObj[this.LTNr].OverridesCount;
        ListClass specialList2Obj = this.SpecialList2Obj;
        let mut detailNr: i32 =  this.DetailNr;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(specialList2Obj, 15, 300, detailNr, game, tHeader: "Graphic overrides", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
        this.SpecialList2Id = this.AddSubPart( tsubpart, 10, 350, 300, 288, 0);
      }
      if (this.DetailNr > -1)
      {
        this.ss = "Remove the of this landscapetype over selected landscapetype";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
          this.killoverrideId = this.AddSubPart( tsubpart, 400, 410, 32, 16, 1);
        }
      }
      this.ss = "Click to add a landscapetype that is overriden by the selected landscapetype";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.addoverrideId = this.AddSubPart( tsubpart, 400, 430, 32, 16, 1);
      }
      this.ss = "Set the Z-order for override.. the higher the score the later another hex will be overriden.";
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.zoverrideId = this.AddSubPart( tsubpart, 400, 450, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("z=" + Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].OverridesZ), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.zoverridetextId = this.AddSubPart( tsubpart1, 450, 450, 200, 20, 0);
      this.specialList3Obj = ListClass::new();
      if (this.game.Data.LandscapeTypeObj[this.LTNr].OverridesCount2 > -1)
      {
        let mut overridesCount2: i32 =  this.game.Data.LandscapeTypeObj[this.LTNr].OverridesCount2;
        for (let mut tdata: i32 =  0; tdata <= overridesCount2; tdata += 1)
          this.specialList3Obj.add(Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].OverridesType2[tdata]) + ") " + this.game.Data.LandscapeTypeObj[this.game.Data.LandscapeTypeObj[this.LTNr].OverridesType2[tdata]].Name, tdata);
        if (this.Detailnr2 > this.game.Data.LandscapeTypeObj[this.LTNr].OverridesCount2)
          this.Detailnr2 = this.game.Data.LandscapeTypeObj[this.LTNr].OverridesCount2;
        ListClass specialList3Obj = this.specialList3Obj;
        let mut detailnr2: i32 =  this.Detailnr2;
        let mut game: GameClass = this.game;
         local3: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local4: Font =  font;
        tsubpart1 =  new ListSubPartClass(specialList3Obj, 15, 300, detailnr2, game, tHeader: "Overrides for ExtraExterior", tbackbitmap: ( local3), bbx: 10, bby: 350, overruleFont: ( local4));
        this.speciallist3id = this.AddSubPart( tsubpart1, 510, 350, 300, 288, 0);
      }
      if (this.Detailnr2 > -1)
      {
        this.ss = "Remove the of this landscapetype over selected landscapetype";
        if (Strings.Len(this.game.Data.MasterFile) == 0)
        {
          tsubpart1 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
          this.killoverrideId2 = this.AddSubPart( tsubpart1, 900, 410, 32, 16, 1);
        }
      }
      this.ss = "Click to add a landscapetype that is overriden by the selected landscapetype";
      if (Strings.Len(this.game.Data.MasterFile) != 0)
        return;
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
      this.addoverrideID2 = this.AddSubPart( tsubpart1, 900, 430, 32, 16, 1);
    }

    pub fn maketabsheetnr4()
    {
      this.CombatListObj = ListClass::new();
      if (this.DetailNr < -1 | this.DetailNr > 99)
        this.DetailNr = -1;
      let mut index: i32 =  0;
      do
      {
        str1: String = "";
        Expression1: String = Conversion.Str( index) + ") " + this.game.Data.TempString[index + 400];
        if (Strings.Len(Expression1) > 20)
          Expression1 = Strings.Left(str1, 15);
        str2: String = str1 + Expression1 + Strings.Space(20 - Strings.Len(Expression1));
        Expression2: String = "AutoEntr=" + Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].DefBonus[index]));
        this.CombatListObj.add(str2 + Expression2 + Strings.Space(15 - Strings.Len(Expression2)) + ("MaxEntr=" + Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].DefBonusMax[index]))), index);
        index += 1;
      }
      while (index <= 99);
      ListClass combatListObj = this.CombatListObj;
      let mut detailNr: i32 =  this.DetailNr;
      let mut game: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(combatListObj, 11, 580, detailNr, game, true, "Entrench per Unitgroup", tbackbitmap: ( local1), bbx: 10, bby: 360, overruleFont: ( local2));
      this.CombatListId = this.AddSubPart( tsubpart, 10, 360, 580, 224, 0);
      if (this.DetailNr <= -1)
        return;
      this.maketabsheetnr4b();
    }

    pub fn maketabsheetnr4b()
    {
      this.ss = "Click to change the AutoEntrench the selected unitgroup will receive in this landscape";
      str1: String = Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].DefBonus[this.DetailNr]));
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.b18id = this.AddSubPart( tsubpart, 610, 340, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("AutoEntrench: " + str1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.b18textid = this.AddSubPart( tsubpart1, 650, 339, 400, 20, 0);
      this.ss = "Click to change the Maximum Entrench level the selected unitgroup can attain in this landscape";
      str2: String = Strings.Trim(Conversion.Str( this.game.Data.LandscapeTypeObj[this.LTNr].DefBonusMax[this.DetailNr]));
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.b19id = this.AddSubPart( tsubpart2, 610, 360, 32, 16, 1);
      }
      let mut tsubpart3: SubPartClass =  TextPartClass::new("MaxEntrench: " + str2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.b19textid = this.AddSubPart( tsubpart3, 650, 359, 400, 20, 0);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.LTListId)
            {
              let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                this.LTNr = num2;
                this.MakeLandscapeTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BAddLTId)
            {
              this.game.Data.AddLandscapeType();
              this.MakeLTListGUI(this.LTNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BCloneLTId)
            {
              if (this.LTNr > -1)
              {
                this.game.Data.AddLandscapeType();
                this.game.Data.LandscapeTypeObj[this.game.Data.LandscapeTypeCounter] = this.game.Data.LandscapeTypeObj[this.LTNr].Clone();
                this.game.Data.LandscapeTypeObj[this.game.Data.LandscapeTypeCounter].LoadSprites();
                let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
                for (let mut index2: i32 =  0; index2 <= sfTypeCounter; index2 += 1)
                {
                  this.game.Data.SFTypeObj[index2].CombatModAtt = (float[]) Utils.CopyArray((Array) this.game.Data.SFTypeObj[index2].CombatModAtt, (Array) new float[this.game.Data.LandscapeTypeCounter + 1]);
                  this.game.Data.SFTypeObj[index2].CombatModDef = (float[]) Utils.CopyArray((Array) this.game.Data.SFTypeObj[index2].CombatModDef, (Array) new float[this.game.Data.LandscapeTypeCounter + 1]);
                  this.game.Data.SFTypeObj[index2].ExtraRecon = (int[]) Utils.CopyArray((Array) this.game.Data.SFTypeObj[index2].ExtraRecon, (Array) new int[this.game.Data.LandscapeTypeCounter + 1]);
                  this.game.Data.SFTypeObj[index2].CombatModAtt[this.game.Data.LandscapeTypeCounter] = this.game.Data.SFTypeObj[index2].CombatModAtt[this.LTNr];
                  this.game.Data.SFTypeObj[index2].CombatModDef[this.game.Data.LandscapeTypeCounter] = this.game.Data.SFTypeObj[index2].CombatModDef[this.LTNr];
                  this.game.Data.SFTypeObj[index2].ExtraRecon[this.game.Data.LandscapeTypeCounter] = this.game.Data.SFTypeObj[index2].ExtraRecon[this.LTNr];
                }
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut num3: i32 =   Interaction.MsgBox( "You have to select an existing LT before you can make a clone.", Title: ( "Shadow Empire : Planetary Conquest"));
            }
            else
            {
              if (num1 == this.BNameId)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BBuildRoadId)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].CanBuildRoad = !this.game.Data.LandscapeTypeObj[this.LTNr].CanBuildRoad;
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e6id)
              {
                ColorDialog colorDialog = ColorDialog::new();
                colorDialog.Color = this.game.Data.LandscapeTypeObj[this.LTNr].Red <= -1 ? Color.FromArgb( byte.MaxValue, 128, 128, 128) : Color.FromArgb( byte.MaxValue, this.game.Data.LandscapeTypeObj[this.LTNr].Red, this.game.Data.LandscapeTypeObj[this.LTNr].Green, this.game.Data.LandscapeTypeObj[this.LTNr].Blue);
                if (colorDialog.ShowDialog() == DialogResult.OK)
                {
                  LandscapeTypeClass landscapeTypeClass1 = this.game.Data.LandscapeTypeObj[this.LTNr];
                  color: Color = colorDialog.Color;
                  let mut r: i32 =   color.R;
                  landscapeTypeClass1.Red = r;
                  LandscapeTypeClass landscapeTypeClass2 = this.game.Data.LandscapeTypeObj[this.LTNr];
                  color = colorDialog.Color;
                  let mut g: i32 =   color.G;
                  landscapeTypeClass2.Green = g;
                  LandscapeTypeClass landscapeTypeClass3 = this.game.Data.LandscapeTypeObj[this.LTNr];
                  color = colorDialog.Color;
                  let mut b1: i32 =   color.B;
                  landscapeTypeClass3.Blue = b1;
                }
                else
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].Red = -1;
                  this.game.Data.LandscapeTypeObj[this.LTNr].Green = -1;
                  this.game.Data.LandscapeTypeObj[this.LTNr].Blue = -1;
                }
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e1id)
              {
                float num4 =  Conversion.Val(Interaction.InputBox("Give Road cost modifier *X (0.5 or 2 or 2.5)", "Shadow Empire : Planetary Conquest"));
                if ( num4 >= 0.0 &  num4 < 1000.0)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].RoadCostModifier = num4;
                }
                else
                {
                  let mut num5: i32 =   Interaction.MsgBox( "between 0-999 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e2id)
              {
                let mut num6: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give AI Block", "Shadow Empire : Planetary Conquest")));
                if (num6 >= 0 & num6 < 1000)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].AIBlock = num6;
                }
                else
                {
                  let mut num7: i32 =   Interaction.MsgBox( "between 0-999 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e86id)
              {
                let mut num8: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Obstruct", "Shadow Empire : Planetary Conquest")));
                if (num8 >= 0 & num8 <= 100)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].Obstruction = num8;
                }
                else
                {
                  let mut num9: i32 =   Interaction.MsgBox( "between 0-100 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e3id)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].CanParadrop = !this.game.Data.LandscapeTypeObj[this.LTNr].CanParadrop;
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.OverIsTopId)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].OverIsTop[this.DetailNr] = !this.game.Data.LandscapeTypeObj[this.LTNr].OverIsTop[this.DetailNr];
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e5id)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].CanAmph = !this.game.Data.LandscapeTypeObj[this.LTNr].CanAmph;
                this.MakeLTListGUI(this.LTNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e4id)
              {
                this.game.HandyFunctionsObj.randomizeLT();
                let mut num10: i32 =   Interaction.MsgBox( "Done");
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e8id)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].Interior = !this.game.Data.LandscapeTypeObj[this.LTNr].Interior;
                this.MakeLandscapeTypeItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e10id)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].ExtraExteriorSame = !this.game.Data.LandscapeTypeObj[this.LTNr].ExtraExteriorSame;
                this.MakeLandscapeTypeItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e11id)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].BlackedOut = !this.game.Data.LandscapeTypeObj[this.LTNr].BlackedOut;
                this.MakeLandscapeTypeItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.c6id)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].Transparent = !this.game.Data.LandscapeTypeObj[this.LTNr].Transparent;
                this.MakeLandscapeTypeItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BSL2Id)
              {
                if (this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer6 = !this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer6;
                }
                else
                {
                  let mut num11: i32 =   Interaction.MsgBox( "You can only set if Special Layer is active", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                if (this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID > -1)
                {
                  BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = (SimpleByteCache) null;
                  BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = false;
                }
                if (this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID > -1)
                {
                  BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = (SimpleByteCache) null;
                  BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = false;
                }
                this.MakeLandscapeTypeItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BSLId)
              {
                this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer = !this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer;
                if (!this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].UseSheet = false;
                  this.game.Data.LandscapeTypeObj[this.LTNr].SheetFileName = "systemgraphics/trans.bmp";
                }
                if (this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer)
                {
                  if (Interaction.MsgBox( "Use full 64 sprites?", MsgBoxStyle.YesNo) == MsgBoxResult.No)
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer6 = true;
                  }
                  else
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].SpecialLayer6 = false;
                    if (Interaction.MsgBox( "Use Fred SpriteSheet?", MsgBoxStyle.YesNo) == MsgBoxResult.No)
                      this.game.Data.LandscapeTypeObj[this.LTNr].UseSheet = false;
                    else
                      this.game.Data.LandscapeTypeObj[this.LTNr].UseSheet = true;
                  }
                  if (!this.game.Data.LandscapeTypeObj[this.LTNr].UseSheet)
                  {
                    if (Interaction.MsgBox( "Auto Load those files?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    {
                      extstring: String = Interaction.InputBox("Give a graphical extension: .jpg, .png, .bmp");
                      dirstring: String = Interaction.InputBox("Give a directory name under the graphics directory", "Shadow Empire : Planetary Conquest");
                      if (File.Exists(this.game.AppPath + "graphics/" + dirstring + "/a1" + extstring))
                      {
                        this.game.Data.LandscapeTypeObj[this.LTNr].SheetFileName = "systemgraphics/trans.bmp";
                        this.game.Data.LandscapeTypeObj[this.LTNr].AutoLoadSpecial(dirstring, extstring);
                        this.MakeLTListGUI(this.LTNr);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      let mut num12: i32 =   Interaction.MsgBox( "Could not find this dir... give it like 'sea' or 'africa/desert', make sure a1 is present.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                  }
                  else
                  {
                    filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select normal sized sheet (big & small will be auto-linked)", this.game.AppPath + "graphics\\", true);
                    if (File.Exists(this.game.AppPath + "graphics/" + filename))
                    {
                      this.game.Data.LandscapeTypeObj[this.LTNr].ReplaceSpriteSheet(filename);
                      this.MakeLTListGUI(this.LTNr);
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    let mut num13: i32 =   Interaction.MsgBox( "Could not find this file... ", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                }
                if (this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID > -1)
                {
                  BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = (SimpleByteCache) null;
                  BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = false;
                }
                if (this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID > -1)
                {
                  BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = (SimpleByteCache) null;
                  BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = false;
                }
                this.MakeLandscapeTypeItemGUI();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.OptionsListId)
              {
                let mut num14: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num14 > -1)
                {
                  this.TabSheetNr = num14;
                  this.maketabsheet();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BRemoveLtId)
              {
                this.game.Data.RemoveLandscapeType(this.LTNr);
                this.LTNr = -1;
                this.MakeLTListGUI(-1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BasicListId)
              {
                let mut num15: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num15 > -1)
                {
                  this.DetailNr = num15;
                  if (this.TabSheetNr == 0)
                    this.maketabsheetnr0b();
                  if (this.TabSheetNr == 5)
                    this.maketabsheetnr5b();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e7id)
              {
                str: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select New Tileset to deconstruct in 62 parts, instead of 64,... g1 is not overwritten. a1 is created empty if not present.", this.game.AppPath + "graphics\\", true);
                if (File.Exists(this.game.AppPath + "graphics/" + str))
                {
                  this.DeconstructTileset(this.game.AppPath + "graphics/" + str);
                }
                else
                {
                  let mut num16: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                this.maketabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e7bid)
              {
                this.ConstructTileset(this.game.HandyFunctionsObj.SaveSomething("Png|*.png", "Select New Tileset to deconstruct in 62 parts, instead of 64,... g1 is not overwritten. a1 is created empty if not present.", this.game.AppPath + "graphics\\", true));
                this.maketabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.e77id)
              {
                if (Interaction.MsgBox( "Are you sure? (keep in mind can take a while)", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                {
                  this.ClearPixelFormat();
                  let mut num17: i32 =   Interaction.MsgBox( "Finished!", Title: ( "Shadow Empire : Planetary Conquest"));
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
              else
              {
                if (num1 == this.i1change)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For new sideways sprite:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplaceSidewaysSprite1(filename, this.DetailNr);
                  }
                  else
                  {
                    let mut num18: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.i2change)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For new sideways 2 sprite:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplaceSidewaysSprite2(filename, this.DetailNr);
                  }
                  else
                  {
                    let mut num19: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.i3change)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For new sideways 3 sprite:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplaceSidewaysSprite3(filename, this.DetailNr);
                  }
                  else
                  {
                    let mut num20: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BAddBasicId)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For New Basic Sprite:", this.game.AppPath + "graphics\\", true);
                  picfilename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For New Basic Picture:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename) & File.Exists(this.game.AppPath + "graphics/" + picfilename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].AddBasicSprite(filename, picfilename);
                  }
                  else
                  {
                    let mut num21: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BChangePreHexPicId)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For New PreHex Sprite:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplacePreHexPicSprite(filename);
                    if (!Information.IsNothing( this.game.Data.LandscapeTypeObj[this.LTNr].TempHexBitmap))
                    {
                      this.game.Data.LandscapeTypeObj[this.LTNr].TempHexBitmap.Dispose();
                      this.game.Data.LandscapeTypeObj[this.LTNr].TempHexBitmap = (Bitmap) null;
                    }
                  }
                  else
                  {
                    let mut num22: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BChangeBasicSpriteId)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For New Basic Sprite:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplaceBasicSprite(this.DetailNr, filename);
                  }
                  else
                  {
                    let mut num23: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BChangeBasicSpriteId2)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For New Basic Sprite:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplaceBasicSprite2(this.DetailNr, filename);
                  }
                  else
                  {
                    let mut num24: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.Bchangebasicspriteid3)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For New Basic Sprite:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplaceBasicSprite3(this.DetailNr, filename);
                  }
                  else
                  {
                    let mut num25: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.e80id)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexTexture = !this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexTexture;
                  if (this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID > -1)
                  {
                    BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = (SimpleByteCache) null;
                    BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = false;
                  }
                  if (this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID > -1)
                  {
                    BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = (SimpleByteCache) null;
                    BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = false;
                  }
                  this.MakeLandscapeTypeItemGUI();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.e82id)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexBorderTexture = !this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexBorderTexture;
                  if (this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID > -1)
                  {
                    BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = (SimpleByteCache) null;
                    BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = false;
                  }
                  if (this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID > -1)
                  {
                    BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = (SimpleByteCache) null;
                    BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = false;
                  }
                  this.MakeLandscapeTypeItemGUI();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.e83id)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexTextureAndRegularPreHex = !this.game.Data.LandscapeTypeObj[this.LTNr].UsePreHexTextureAndRegularPreHex;
                  this.MakeLandscapeTypeItemGUI();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.e81id)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For New Texture:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplacePreHexTexture(filename);
                  }
                  else
                  {
                    let mut num26: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  if (this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID > -1)
                  {
                    BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = (SimpleByteCache) null;
                    BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].PreHexTextureID] = false;
                  }
                  if (this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID > -1)
                  {
                    BitmapStore.simpleByteCacheObj[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = (SimpleByteCache) null;
                    BitmapStore.simpleByteCacheSet[this.game.Data.LandscapeTypeObj[this.LTNr].SheetSpriteID] = false;
                  }
                  this.MakeLandscapeTypeItemGUI();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BRandomId)
                {
                  let mut num27: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new random value >0. -1=none", "Shadow Empire : Planetary Conquest")));
                  if (num27 >= -1 & num27 < 999)
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].BasicSpriteRandom[this.DetailNr] = num27;
                  }
                  else
                  {
                    let mut num28: i32 =   Interaction.MsgBox( "between -1 and 999 plz.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BPlotLastId)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].PlotLast[this.DetailNr] = !this.game.Data.LandscapeTypeObj[this.LTNr].PlotLast[this.DetailNr];
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.e84id)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].PlotBeforeRiver[this.DetailNr] = !this.game.Data.LandscapeTypeObj[this.LTNr].PlotBeforeRiver[this.DetailNr];
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BChangeBasicPicId)
                {
                  filename: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp|Jpg|*.jpg", "Select File For New Basic Picture:", this.game.AppPath + "graphics\\", true);
                  if (File.Exists(this.game.AppPath + "graphics/" + filename))
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].ReplacePicSprite(this.DetailNr, filename);
                  }
                  else
                  {
                    let mut num29: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BRemoveBasicId)
                {
                  this.game.Data.RemoveLandscapeBasicSprite(this.LTNr, this.DetailNr);
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.BDrawId)
                {
                  this.game.EditObj.PencilType = 1;
                  this.game.EditObj.PencilData1 = this.LTNr;
                  this.game.EditObj.PencilData2 = this.DetailNr;
                  windowReturnClass.AddCommand(4, 13);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.SpecialListId)
                {
                  let mut num30: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  this.SubPartFlag[index1] = true;
                  if (num30 > -1)
                  {
                    this.DetailNr = num30;
                    this.maketabsheetnr2b();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.SpecialList2Id)
                {
                  let mut num31: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  this.SubPartFlag[index1] = true;
                  if (num31 > -1)
                  {
                    this.DetailNr = num31;
                    this.maketabsheetnr3();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.speciallist3id)
                {
                  let mut num32: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  this.SubPartFlag[index1] = true;
                  if (num32 > -1)
                  {
                    this.Detailnr2 = num32;
                    this.maketabsheetnr3();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.zoverrideId)
                {
                  let mut num33: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new Z-Override Value (899=the sprites of this lt are drawn after overdraw, >900=even overdraws)", "Shadow Empire : Planetary Conquest")));
                  if (num33 >= -1 & num33 <= 999)
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].OverridesZ = num33;
                  }
                  else
                  {
                    let mut num34: i32 =   Interaction.MsgBox( "between 0-999 plz.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  this.maketabsheetnr3();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.killoverrideId)
                {
                  this.game.Data.LandscapeTypeObj[this.LTNr].RemoveOverride(this.DetailNr);
                  this.maketabsheetnr3();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.addoverrideId)
                {
                  Form3::new( this.formref).Initialize(this.game.Data, 2, this.LTNr);
                  this.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.killoverrideId2)
                {
                  if (this.Detailnr2 > -1)
                  {
                    this.game.Data.LandscapeTypeObj[this.LTNr].RemoveOverride2(this.Detailnr2);
                    this.maketabsheetnr3();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else
                {
                  if (num1 == this.addoverrideID2)
                  {
                    Form3::new( this.formref).Initialize(this.game.Data, 62, this.LTNr);
                    this.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.TxtId)
                  {
                    Form2::new( this.formref).Initialize(this.game.Data, 10, this.LTNr);
                    this.MakeLandscapeTypeItemGUI();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.e9id)
                  {
                    Form3::new( this.formref).Initialize(this.game.Data, 61, this.LTNr);
                    this.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.MoveListId)
                  {
                    let mut num35: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                    this.SubPartFlag[index1] = true;
                    if (num35 > -1)
                    {
                      this.DetailNr = num35;
                      this.maketabsheetnr1b();
                    }
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.BChangeSpecialSpriteId)
                  {
                    filename: String = Interaction.InputBox("Give File Name For Replacement of selected Special Sprite #" + Conversion.Str( (this.DetailNr + 1)) + ":", "Shadow Empire : Planetary Conquest");
                    if (File.Exists(this.game.AppPath + "graphics/" + filename))
                    {
                      this.game.Data.LandscapeTypeObj[this.LTNr].ReplaceSpecialSprite(this.DetailNr + 1, filename);
                    }
                    else
                    {
                      let mut num36: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    this.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.BChangeMoveCostId)
                  {
                    let mut num37: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Move cost#" + Conversion.Str( this.DetailNr) + ":", "Shadow Empire : Planetary Conquest")));
                    if (num37 > -1 & num37 < 10000)
                      this.game.Data.LandscapeTypeObj[this.LTNr].MoveCost[this.DetailNr] = num37;
                    this.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.e14id)
                  {
                    if (Interaction.MsgBox( "Are you sure? Changes will be irreversible. All (32xX)x24, (64xX)x48 and (128xX)x96 sized .png files will be changed. Also all fred sprite sheets will be changed. formats: 192x264, 384x528 and 768x1056 ", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    {
                      str: String = this.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File. The directory it is in and all its subdirectories (1 level deep only) will be checked.", this.game.AppPath, false);
                      if (File.Exists(str))
                      {
                        let mut num38: i32 =   Interaction.MsgBox( "Ok hold on... this can take some time.", Title: ( "Shadow Empire : Planetary Conquest"));
                        this.ClearPixels(str);
                      }
                      else
                      {
                        let mut num39: i32 =   Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      this.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    this.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 != this.e14id)
                  {
                    if (num1 == this.e12id)
                    {
                      let mut num40: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give LTnr to use for borders for prehex sprite. -1=dont use", "Shadow Empire : Planetary Conquest")));
                      if (num40 >= -1 & num40 <= this.game.Data.LandscapeTypeCounter)
                        this.game.Data.LandscapeTypeObj[this.LTNr].PreHexBorder = num40;
                      this.MakeLandscapeTypeItemGUI();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.c4id)
                    {
                      let mut num41: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give LTnr to use. -1=dont use", "Shadow Empire : Planetary Conquest")));
                      if (num41 >= -1 & num41 <= this.game.Data.LandscapeTypeCounter)
                        this.game.Data.LandscapeTypeObj[this.LTNr].NavyOverride = num41;
                      this.MakeLandscapeTypeItemGUI();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.c5id)
                    {
                      let mut num42: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give LTnr to use. -1=dont use", "Shadow Empire : Planetary Conquest")));
                      if (num42 >= -1 & num42 <= this.game.Data.LandscapeTypeCounter)
                        this.game.Data.LandscapeTypeObj[this.LTNr].AirOverride = num42;
                      this.MakeLandscapeTypeItemGUI();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.e13id)
                    {
                      this.game.Data.LandscapeTypeObj[this.LTNr].DontShowInList = !this.game.Data.LandscapeTypeObj[this.LTNr].DontShowInList;
                      this.MakeLandscapeTypeItemGUI();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.e15id)
                    {
                      this.game.Data.LandscapeTypeObj[this.LTNr].NoPortReq = !this.game.Data.LandscapeTypeObj[this.LTNr].NoPortReq;
                      this.MakeLandscapeTypeItemGUI();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.e85id)
                    {
                      this.game.Data.LandscapeTypeObj[this.LTNr].usePreHexBorderOwnZ = !this.game.Data.LandscapeTypeObj[this.LTNr].usePreHexBorderOwnZ;
                      this.MakeLandscapeTypeItemGUI();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.e87id)
                    {
                      this.game.Data.LandscapeTypeObj[this.LTNr].FuzzyOwnerAssured = !this.game.Data.LandscapeTypeObj[this.LTNr].FuzzyOwnerAssured;
                      this.MakeLandscapeTypeItemGUI();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.BBuildGroundId)
                    {
                      Form3::new( this.formref).Initialize(this.game.Data, 1, this.LTNr, this.game.Data.LandscapeTypeObj[this.LTNr].BuildGround);
                      this.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.BIsSeaId)
                    {
                      this.game.Data.LandscapeTypeObj[this.LTNr].IsSea = !this.game.Data.LandscapeTypeObj[this.LTNr].IsSea;
                      this.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.CombatListId)
                    {
                      let mut num43: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                      this.SubPartFlag[index1] = true;
                      if (num43 > -1)
                      {
                        this.DetailNr = num43;
                        this.maketabsheet();
                      }
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.b18id)
                    {
                      float num44 =  Conversion.Val(Interaction.InputBox("Give new auto-entrench.", "Shadow Empire : Planetary Conquest"));
                      if ( num44 < 0.0 |  num44 > 999.0)
                      {
                        let mut num45: i32 =   Interaction.MsgBox( "Between 0 and 999.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      else
                        this.game.Data.LandscapeTypeObj[this.LTNr].DefBonus[this.DetailNr] = num44;
                      this.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.b19id)
                    {
                      float num46 =  Conversion.Val(Interaction.InputBox("Give new max-entrench.", "Shadow Empire : Planetary Conquest"));
                      if ( num46 < 0.0 |  num46 > 999.0)
                      {
                        let mut num47: i32 =   Interaction.MsgBox( "Between 0 and 999.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      else
                        this.game.Data.LandscapeTypeObj[this.LTNr].DefBonusMax[this.DetailNr] = num46;
                      this.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.c3id)
                    {
                      let mut num48: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give new hidepts.", "Shadow Empire : Planetary Conquest")));
                      float num49;
                      if (num48 < 0 |  num49 > 999.0)
                      {
                        let mut num50: i32 =   Interaction.MsgBox( "Between 0 and 999 please.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      else
                        this.game.Data.LandscapeTypeObj[this.LTNr].HidePts = num48;
                      this.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                  }
                }
              }
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn ConstructTileset2(s: String)
    {
      strArray: Vec<String> = new string[65];
      bitmap: Bitmap = new Bitmap(1280, 1280);
      bitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics objGraphics = Graphics.FromImage((Image) bitmap);
      objGraphics.CompositingMode = CompositingMode.SourceCopy;
      let mut num1: i32 =  131;
      let mut num2: i32 =  99;
      let mut index: i32 =  1;
      do
      {
        if (this.game.Data.LandscapeTypeObj[this.LTNr].LayerSpriteID[index] > 0)
          objGraphics.DrawImage((Image) BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[this.LTNr].LayerSpriteID[index], 1), 2 + this.game.SHEETX[index] * num1, 2 + this.game.SHEETY[index] * num2);
        index += 1;
      }
      while (index <= 64);
      objGraphics.DrawImage((Image) BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[this.LTNr].PreHexPicID, 1), 2, 2 + 11 * num2);
      let mut num3: i32 =  0;
      do
      {
        DrawMod.drawLine( objGraphics, num3 * num1, 0, num3 * num1, 2 + 11 * num2 - 2, 0,  byte.MaxValue, 0,  byte.MaxValue);
        num3 += 1;
      }
      while (num3 <= 6);
      let mut num4: i32 =  0;
      do
      {
        DrawMod.drawLine( objGraphics, 0, num4 * num2, 2 + 6 * num1 - 2, num4 * num2, 0,  byte.MaxValue, 0,  byte.MaxValue);
        num4 += 1;
      }
      while (num4 <= 11);
      DrawMod.drawLine( objGraphics, 0, 12 * num2, 2 + 12 * num1 - 2, 12 * num2, 0,  byte.MaxValue, 0,  byte.MaxValue);
      FileStream fileStream = new FileStream(s, FileMode.Create);
      bitmap.Save((Stream) fileStream, ImageFormat.Png);
      fileStream.Close();
      objGraphics.Dispose();
      bitmap.Dispose();
    }

    pub fn ConstructTileset(s: String)
    {
      strArray: Vec<String> = new string[65];
      if (Interaction.MsgBox( "Make alternate vicFormat?", MsgBoxStyle.YesNo) == MsgBoxResult.Yes)
      {
        this.ConstructTileset2(s);
      }
      else
      {
        bitmap1: Bitmap = new Bitmap(384, 528);
        bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics1 = Graphics.FromImage((Image) bitmap1);
        graphics1.CompositingMode = CompositingMode.SourceCopy;
        let mut num1: i32 =  64;
        let mut num2: i32 =  48;
        let mut index1: i32 =  1;
        do
        {
          graphics1.DrawImage((Image) BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[this.LTNr].LayerSpriteID[index1]), this.game.SHEETX[index1] * num1, this.game.SHEETY[index1] * num2);
          index1 += 1;
        }
        while (index1 <= 64);
        FileStream fileStream1 = new FileStream(this.game.AppPath + "graphics/" + s, FileMode.Create);
        bitmap1.Save((Stream) fileStream1, ImageFormat.Png);
        fileStream1.Close();
        graphics1.Dispose();
        bitmap2: Bitmap = new Bitmap(768, 1056);
        bitmap2.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics2 = Graphics.FromImage((Image) bitmap2);
        graphics2.CompositingMode = CompositingMode.SourceCopy;
        let mut num3: i32 =  128;
        let mut num4: i32 =  96;
        let mut index2: i32 =  1;
        do
        {
          graphics2.DrawImage((Image) BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[this.LTNr].LayerSpriteID[index2], 1), this.game.SHEETX[index2] * num3, this.game.SHEETY[index2] * num4);
          index2 += 1;
        }
        while (index2 <= 64);
        FileStream fileStream2 = new FileStream(this.game.AppPath + "graphics/" + s.Replace(".png", "_big.png"), FileMode.Create);
        bitmap2.Save((Stream) fileStream2, ImageFormat.Png);
        fileStream2.Close();
        graphics2.Dispose();
        bitmap2.Dispose();
      }
    }

    pub fn DeconstructTileset(s: String)
    {
      strArray: Vec<String> = new string[65];
      let mut num1: i32 =  1;
      num2: i32;
      do
      {
        num2 = num1;
        num1 = Strings.InStr(num1 + 1, s, "\\");
      }
      while (num1 > 0);
      str1: String = Strings.Left(s, num2 - 1);
      str2: String = ".png";
      strArray[1] = str1 + "/a1" + str2;
      strArray[2] = str1 + "/b1" + str2;
      strArray[3] = str1 + "/b2" + str2;
      strArray[4] = str1 + "/b3" + str2;
      strArray[5] = str1 + "/b4" + str2;
      strArray[6] = str1 + "/b5" + str2;
      strArray[7] = str1 + "/b6" + str2;
      strArray[8] = str1 + "/c1" + str2;
      strArray[9] = str1 + "/c7" + str2;
      strArray[10] = str1 + "/c13" + str2;
      strArray[11] = str1 + "/c11" + str2;
      strArray[12] = str1 + "/c6" + str2;
      strArray[13] = str1 + "/c2" + str2;
      strArray[14] = str1 + "/c8" + str2;
      strArray[15] = str1 + "/c14" + str2;
      strArray[16] = str1 + "/c12" + str2;
      strArray[17] = str1 + "/c3" + str2;
      strArray[18] = str1 + "/c9" + str2;
      strArray[19] = str1 + "/c15" + str2;
      strArray[20] = str1 + "/c4" + str2;
      strArray[21] = str1 + "/c10" + str2;
      strArray[22] = str1 + "/c5" + str2;
      strArray[23] = str1 + "/d1" + str2;
      strArray[24] = str1 + "/d7" + str2;
      strArray[25] = str1 + "/d13" + str2;
      strArray[26] = str1 + "/d6" + str2;
      strArray[27] = str1 + "/d15" + str2;
      strArray[28] = str1 + "/d19" + str2;
      strArray[29] = str1 + "/d12" + str2;
      strArray[30] = str1 + "/d10" + str2;
      strArray[31] = str1 + "/d18" + str2;
      strArray[32] = str1 + "/d5" + str2;
      strArray[33] = str1 + "/d2" + str2;
      strArray[34] = str1 + "/d8" + str2;
      strArray[35] = str1 + "/d14" + str2;
      strArray[36] = str1 + "/d16" + str2;
      strArray[37] = str1 + "/d20" + str2;
      strArray[38] = str1 + "/d11" + str2;
      strArray[39] = str1 + "/d3" + str2;
      strArray[40] = str1 + "/d9" + str2;
      strArray[41] = str1 + "/d17" + str2;
      strArray[42] = str1 + "/d4" + str2;
      strArray[43] = str1 + "/e5" + str2;
      strArray[44] = str1 + "/e10" + str2;
      strArray[45] = str1 + "/e4" + str2;
      strArray[46] = str1 + "/e15" + str2;
      strArray[47] = str1 + "/e9" + str2;
      strArray[48] = str1 + "/e3" + str2;
      strArray[49] = str1 + "/e6" + str2;
      strArray[50] = str1 + "/e11" + str2;
      strArray[51] = str1 + "/e13" + str2;
      strArray[52] = str1 + "/e7" + str2;
      strArray[53] = str1 + "/e12" + str2;
      strArray[54] = str1 + "/e1" + str2;
      strArray[55] = str1 + "/e14" + str2;
      strArray[56] = str1 + "/e8" + str2;
      strArray[57] = str1 + "/e2" + str2;
      strArray[58] = str1 + "/f6" + str2;
      strArray[59] = str1 + "/f5" + str2;
      strArray[60] = str1 + "/f4" + str2;
      strArray[61] = str1 + "/f3" + str2;
      strArray[62] = str1 + "/f2" + str2;
      strArray[63] = str1 + "/f1" + str2;
      strArray[64] = str1 + "/g1" + str2;
      bitmap1: Bitmap = new Bitmap(s);
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      bitmap2: Bitmap;
      width: i32;
      height: i32;
      if (bitmap1.Width < 500)
      {
        bitmap2 = new Bitmap(64, 48);
        width = 64;
        height = 48;
      }
      else
      {
        bitmap2 = new Bitmap(128, 96);
        width = 128;
        height = 96;
      }
      bitmap2.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics = Graphics.FromImage((Image) bitmap2);
      graphics.CompositingMode = CompositingMode.SourceCopy;
      if (!File.Exists(strArray[1]))
      {
        graphics.Clear(Color.Transparent);
        FileStream fileStream = new FileStream(strArray[1], FileMode.Create);
        bitmap2.Save((Stream) fileStream, ImageFormat.Png);
        fileStream.Close();
      }
      let mut num3: i32 =  0;
      do
      {
        let mut num4: i32 =  0;
        do
        {
          graphics.DrawImage((Image) bitmap1, Rectangle::new(0, 0, width, height), Rectangle::new(num4 * width, num3 * height, width, height), GraphicsUnit.Pixel);
          let mut index: i32 =  2 + num3 * 6 + num4;
          if (File.Exists(strArray[index]))
            File.Delete(strArray[index]);
          FileStream fileStream = new FileStream(strArray[index], FileMode.Create);
          bitmap2.Save((Stream) fileStream, ImageFormat.Png);
          fileStream.Close();
          if (!(num3 == 10 & num4 > 1))
            num4 += 1;
          else
            break;
        }
        while (num4 <= 5);
        num3 += 1;
      }
      while (num3 <= 10);
    }

    pub fn ClearPixels(s: String)
    {
      s = s.Replace("\\", "/");
      let mut num1: i32 =  0;
      while (Strings.InStr(num1 + 1, s, "/") > 0)
        num1 = Strings.InStr(num1 + 1, s, "/");
      s = Strings.Left(s, num1 - 1);
      let mut Number1: i32 =  0;
      let mut Number2: i32 =  0;
      DirectoryInfo directoryInfo = new DirectoryInfo(s);
      foreach (FileInfo file in directoryInfo.GetFiles("*.png"))
      {
        if (Strings.InStr(file.Name, ".png") > 0)
        {
          Number1 += 1;
          this.ClearPixelsOperation(s + "/" + file.Name);
        }
      }
      foreach (DirectoryInfo directory in directoryInfo.GetDirectories())
      {
        Number2 += 1;
        foreach (FileInfo file in new DirectoryInfo(s + "/" + directory.Name).GetFiles("*.png"))
        {
          if (Strings.InStr(directory.Name, ".png") > 0)
          {
            Number1 += 1;
            this.ClearPixelsOperation(s + "/" + directory.Name + "/" + file.Name);
          }
        }
      }
      let mut num2: i32 =   Interaction.MsgBox( ("Finished. Revised " + Strings.Trim(Conversion.Str( Number1)) + " files in " + Strings.Trim(Conversion.Str( Number2)) + " directories."), Title: ( "Shadow Empire : Planetary Conquest"));
    }

    pub fn ClearPixelsOperation(s: String)
    {
      bitmap1: Bitmap = (Bitmap) Image.FromStream((Stream) new MemoryStream(File.ReadAllBytes(s)));
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      bitmap2: Bitmap;
      num1: i32;
      if (bitmap1.Width % 32 == 0 & bitmap1.Height == 24)
      {
        bitmap2 = BitmapStore.GetBitmap(this.game.SHADEDHEX, -1);
        num1 = 32;
      }
      else if (bitmap1.Width % 64 == 0 & bitmap1.Height == 48)
      {
        bitmap2 = BitmapStore.GetBitmap(this.game.SHADEDHEX);
        num1 = 64;
      }
      else if (bitmap1.Width % 128 == 0 & bitmap1.Height == 96)
      {
        bitmap3: Bitmap = BitmapStore.GetBitmap(this.game.SHADEDHEX, 1);
        let mut num2: i32 =  128;
        let mut num3: i32 =  bitmap3.Height - 1;
        for (let mut y: i32 =  0; y <= num3; y += 1)
        {
          let mut num4: i32 =  bitmap1.Width - 1;
          for (let mut x1: i32 =  0; x1 <= num4; x1 += 1)
          {
            let mut x2: i32 =  (num2 + x1) % num2;
            pixel1: Color = bitmap3.GetPixel(x2, y);
            pixel2: Color;
            if (x2 == 0)
            {
              pixel2 = bitmap1.GetPixel(x1, y);
              if (pixel2.A > (byte) 80 & pixel2.R > (byte) 100 & pixel2.B > (byte) 100 & pixel2.G > (byte) 100)
              {
                pixel1 = bitmap1.GetPixel(x1 + 1, y);
                if (pixel1.A <= (byte) 5)
                  bitmap1.SetPixel(x1, y, Color.FromArgb(0, 0, 0, 0));
              }
              else if (pixel1.A == (byte) 0)
                bitmap1.SetPixel(x1, y, Color.FromArgb(0, 0, 0, 0));
            }
            else if (x2 == num2 - 1)
            {
              pixel2 = bitmap1.GetPixel(x1, y);
              if (pixel2.A > (byte) 80 & pixel2.R > (byte) 100 & pixel2.B > (byte) 100 & pixel2.G > (byte) 100)
              {
                pixel1 = bitmap1.GetPixel(x1 - 1, y);
                if (pixel1.A <= (byte) 5)
                  bitmap1.SetPixel(x1, y, Color.FromArgb(0, 0, 0, 0));
              }
              else if (pixel1.A == (byte) 0)
                bitmap1.SetPixel(x1, y, Color.FromArgb(0, 0, 0, 0));
            }
            else if (pixel1.A == (byte) 0)
              bitmap1.SetPixel(x1, y, Color.FromArgb(0, 0, 0, 0));
          }
        }
      }
      else
      {
        bitmap4: Bitmap;
        num5: i32;
        num6: i32;
        if (bitmap1.Width == 192 & bitmap1.Height == 264)
        {
          bitmap4 = BitmapStore.GetBitmap(this.game.SHADEDHEX, -1);
          num5 = 32;
          num6 = 24;
        }
        else if (bitmap1.Width == 384 & bitmap1.Height == 528)
        {
          bitmap4 = BitmapStore.GetBitmap(this.game.SHADEDHEX);
          num5 = 64;
          num6 = 48;
        }
        else if (bitmap1.Width == 768 & bitmap1.Height == 1056)
        {
          bitmap4 = BitmapStore.GetBitmap(this.game.SHADEDHEX, 1);
          num5 = 128;
          num6 = 96;
        }
        else if (bitmap1.Width >= 768 & bitmap1.Height >= 1056 & bitmap1.Width % 128 == 0 & bitmap1.Height % 96 == 0)
        {
          bitmap4 = BitmapStore.GetBitmap(this.game.SHADEDHEX, 1);
          num5 = 128;
          num6 = 96;
        }
        else
        {
          if (!(bitmap1.Width >= 768 & bitmap1.Height == 1200 & bitmap1.Width % 128 == 0))
            return;
          bitmap4 = BitmapStore.GetBitmap(this.game.SHADEDHEX, 1);
          num5 = 128;
          num6 = 96;
        }
        let mut num7: i32 =  bitmap4.Height - 1;
        for (let mut y1: i32 =  0; y1 <= num7; y1 += 1)
        {
          let mut num8: i32 =  bitmap1.Width - 1;
          for (let mut x3: i32 =  0; x3 <= num8; x3 += 1)
          {
            let mut x4: i32 =  (num5 + x3) % num5;
            let mut y2: i32 =  (num6 + y1) % num6;
            pixel3: Color = bitmap4.GetPixel(x4, y2);
            pixel4: Color;
            if (pixel3.A == (byte) 0)
              bitmap1.SetPixel(x3, y1, Color.FromArgb(0, 0, 0, 0));
            else if (x4 == 0)
            {
              pixel4 = bitmap1.GetPixel(x3, y1);
              if (pixel4.A > (byte) 80 & pixel4.R > (byte) 100 & pixel4.B > (byte) 100 & pixel4.G > (byte) 100)
              {
                pixel3 = bitmap1.GetPixel(x3 + 1, y1);
                if (pixel3.A <= (byte) 15)
                  bitmap1.SetPixel(x3, y1, Color.FromArgb(0, 0, 0, 0));
              }
              else if (pixel3.A == (byte) 0)
                bitmap1.SetPixel(x3, y1, Color.FromArgb(0, 0, 0, 0));
            }
            else if (x4 == num5 - 1)
            {
              pixel4 = bitmap1.GetPixel(x3, y1);
              if (pixel4.A > (byte) 80 & pixel4.R > (byte) 100 & pixel4.B > (byte) 100 & pixel4.G > (byte) 100)
              {
                pixel3 = bitmap1.GetPixel(x3 - 1, y1);
                if (pixel3.A <= (byte) 15)
                  bitmap1.SetPixel(x3, y1, Color.FromArgb(0, 0, 0, 0));
              }
              else if (pixel3.A == (byte) 0)
                bitmap1.SetPixel(x3, y1, Color.FromArgb(0, 0, 0, 0));
            }
            else if (pixel3.A == (byte) 0)
              bitmap1.SetPixel(x3, y1, Color.FromArgb(0, 0, 0, 0));
          }
        }
      }
      if (File.Exists(s))
        File.Delete(s);
      FileStream fileStream = new FileStream(s, FileMode.Create);
      bitmap1.Save((Stream) fileStream, ImageFormat.Png);
      fileStream.Close();
      bitmap1.Dispose();
      bitmap2 = (Bitmap) null;
    }

    pub fn ClearPixelFormat()
    {
      let mut counter: i32 =  BitmapStore.Counter;
      Number: i32;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        str1: String = this.game.AppPath + "graphics/" + BitmapStore.tmpFileName[index];
        if (File.Exists(str1))
          this.SetPixelFormatOperations(str1);
        str2: String = this.game.AppPath + "graphics/" + BitmapStore.MakeBigString(BitmapStore.tmpFileName[index]);
        if (File.Exists(str2))
          this.SetPixelFormatOperations(str2);
        str3: String = this.game.AppPath + "graphics/" + BitmapStore.MakeSmallString(BitmapStore.tmpFileName[index]);
        if (File.Exists(str3))
          this.SetPixelFormatOperations(str3);
        Number += 1;
      }
      let mut num: i32 =   Interaction.MsgBox( ("Finished. Revised " + Strings.Trim(Conversion.Str( Number)) + " files in "), Title: ( "Shadow Empire : Planetary Conquest"));
    }

    pub fn SetPixelFormatOperations(s: String)
    {
      bitmap1: Bitmap = (Bitmap) Image.FromStream((Stream) new MemoryStream(File.ReadAllBytes(s)));
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      bitmap2: Bitmap = new Bitmap(bitmap1.Width, bitmap1.Height, PixelFormat.Format32bppPArgb);
      Graphics graphics = Graphics.FromImage((Image) bitmap2);
      graphics.DrawImageUnscaled((Image) bitmap1, 0, 0);
      graphics.Dispose();
      try
      {
        if (File.Exists(s))
          File.Delete(s);
        FileStream fileStream = new FileStream(s, FileMode.Create);
        if (Strings.InStr(s, ".png") > 0)
          bitmap2.Save((Stream) fileStream, ImageFormat.Png);
        else if (Strings.InStr(s, ".bmp") > 0)
          bitmap2.Save((Stream) fileStream, ImageFormat.Bmp);
        else if (Strings.InStr(s, ".jpg") > 0)
          bitmap2.Save((Stream) fileStream, ImageFormat.Jpeg);
        else
          bitmap2.Save((Stream) fileStream, ImageFormat.Png);
        fileStream.Close();
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      bitmap1.Dispose();
      bitmap2.Dispose();
    }
  }
}
