// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RegimeWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class RegimeWindowClass : WindowClass
  {
     LibListId: i32;
     LibNr: i32;
     ListClass LibListObj;
     regimeListId: i32;
     ListClass regimeListObj;
     BAddregimeId: i32;
     BAddregimeTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     BRedId: i32;
     BRedTextId: i32;
     BRed2Id: i32;
     BRed2TextId: i32;
     BRed3Id: i32;
     BRed3TextId: i32;
     BRed4Id: i32;
     BRed4TextId: i32;
     BMorId: i32;
     BMorTextId: i32;
     BAIId: i32;
     BAITextID: i32;
     PbemId: i32;
     PbemTextId: i32;
     BRemoveregimeId: i32;
     BRemoveregimeTextId: i32;
     BDrawId: i32;
     BDrawTextId: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     BResId: i32;
     BResTextId: i32;
     BPolId: i32;
     BPolTextId: i32;
     clid: i32;
     cltextid: i32;
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
     altid: i32;
     alttextid: i32;
     B1Id: i32;
     B1TextId: i32;
     BSymbolId: i32;
     BChangeSymbolId: i32;
     NatIconId: i32;
     NatIconPic: i32;
     RoundelId: i32;
     RoundelPic: i32;
     stringListId: i32;
     ListClass stringListObj;
     B2Id: i32;
     B2TextId: i32;
     ResListId: i32;
     ListClass ResListObj;
     b3Id: i32;
     B3TextId: i32;
     b5Id: i32;
     B5TextId: i32;
     dipListId: i32;
     ListClass dipListObj;
     b4Id: i32;
     B4TextId: i32;
     regimeNr: i32;
     TabSheetNr: i32;
     DetailNr: i32;
     HighId: i32;
     LowId: i32;
     ss: String;

    pub RegimeWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Regimes")
    {
      self.regimeNr = -1;
      self.TabSheetNr = -1;
      self.LibNr = -1;
      self.DetailNr = -1;
      self.MakeregimeListGUI(-1);
    }

    pub fn DoRefresh()
    {
      if (self.regimeNr > -1)
        self.LibNr = self.game.Data.RegimeObj[self.regimeNr].libId.libSlot;
      self.MakeregimeListGUI(self.regimeNr);
    }

     void MakeregimeListGUI(tregimenr: i32)
    {
      if (self.regimeListId > 0)
        self.RemoveSubPart(self.regimeListId);
      if (self.LibListId > 0)
        self.RemoveSubPart(self.LibListId);
      self.LibListObj = ListClass::new();
      self.LibListObj.add("All", -2);
      let mut num1: i32 = -1;
      let mut num2: i32 = 0;
      let mut libraryCounter: i32 = self.game.Data.LibraryCounter;
      for (let mut index: i32 = 0; index <= libraryCounter; index += 1)
      {
        num2 += 1;
        if (self.LibNr == index)
          num1 = num2;
        self.LibListObj.add(Conversion.Str( index) + ") " + self.game.Data.LibraryObj[index].name, index);
      }
      if (self.LibNr == -1)
        num1 = 0;
      ListClass libListObj = self.LibListObj;
      let mut tlistselect1: i32 = num1;
      let mut game1: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font1: Font =  null;
       local2: Font =  font1;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(libListObj, 7, 200, tlistselect1, game1, tHeader: "Libraries", tbackbitmap: ( local1), bbx: 10, bby: 40, overruleFont: ( local2));
      self.LibListId = self.AddSubPart( tsubpart1, 10, 40, 200, 160, 0);
      let mut num3: i32 = -1;
      let mut num4: i32 = -1;
      if (self.game.Data.RegimeCounter > -1)
      {
        self.regimeListObj = ListClass::new();
        let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
        for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
        {
          if (self.game.Data.RegimeObj[index].libId.libSlot == self.LibNr | self.LibNr == -1)
          {
            self.regimeListObj.add(Strings.Trim(Conversion.Str( index)) + ") " + self.game.Data.RegimeObj[index].Name + "(id=" + self.game.Data.RegimeObj[index].id.ToString() + ")", index);
            num3 += 1;
            if (self.LibNr == index)
              num4 = num3;
          }
        }
        ListClass regimeListObj = self.regimeListObj;
        let mut tlistselect2: i32 = num4;
        let mut game2: GameClass = self.game;
         local3: Bitmap =  self.OwnBitmap;
        font2: Font =  null;
         local4: Font =  font2;
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(regimeListObj, 3, 200, tlistselect2, game2, tHeader: "Regimes", tbackbitmap: ( local3), bbx: 10, bby: 210, overruleFont: ( local4));
        self.regimeListId = self.AddSubPart( tsubpart2, 10, 210, 200, 80, 0);
        self.regimeNr = tregimenr;
        self.MakeregimeTypeItemGUI();
      }
      else
      {
        self.regimeNr = tregimenr;
        self.MakeregimeTypeItemGUI();
      }
      if (self.BAddregimeId > 0)
        self.RemoveSubPart(self.BAddregimeId);
      if (self.BAddregimeTextId > 0)
        self.RemoveSubPart(self.BAddregimeTextId);
      self.ss = "Click to add a regime";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
      self.BAddregimeId = self.AddSubPart( tsubpart3, 610, 70, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("Add Regime", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.BAddregimeTextId = self.AddSubPart( tsubpart4, 650, 69, 300, 20, 0);
    }

     void MakeregimeTypeItemGUI()
    {
      if (self.BNameId > 0)
        self.RemoveSubPart(self.BNameId);
      if (self.BNameTextId > 0)
        self.RemoveSubPart(self.BNameTextId);
      if (self.BRedId > 0)
        self.RemoveSubPart(self.BRedId);
      if (self.BRedTextId > 0)
        self.RemoveSubPart(self.BRedTextId);
      if (self.clid > 0)
        self.RemoveSubPart(self.clid);
      if (self.cltextid > 0)
        self.RemoveSubPart(self.cltextid);
      if (self.BRed2Id > 0)
        self.RemoveSubPart(self.BRed2Id);
      if (self.BRed2TextId > 0)
        self.RemoveSubPart(self.BRed2TextId);
      if (self.BRed3Id > 0)
        self.RemoveSubPart(self.BRed3Id);
      if (self.BRed3TextId > 0)
        self.RemoveSubPart(self.BRed3TextId);
      if (self.BRed4Id > 0)
        self.RemoveSubPart(self.BRed4Id);
      if (self.BRed4TextId > 0)
        self.RemoveSubPart(self.BRed4TextId);
      if (self.BRemoveregimeId > 0)
        self.RemoveSubPart(self.BRemoveregimeId);
      if (self.BRemoveregimeTextId > 0)
        self.RemoveSubPart(self.BRemoveregimeTextId);
      if (self.BDrawId > 0)
        self.RemoveSubPart(self.BDrawId);
      if (self.BDrawTextId > 0)
        self.RemoveSubPart(self.BDrawTextId);
      if (self.OptionsListId > 0)
        self.RemoveSubPart(self.OptionsListId);
      if (self.BMorId > 0)
        self.RemoveSubPart(self.BMorId);
      if (self.BMorTextId > 0)
        self.RemoveSubPart(self.BMorTextId);
      if (self.altid > 0)
        self.RemoveSubPart(self.altid);
      if (self.alttextid > 0)
        self.RemoveSubPart(self.alttextid);
      if (self.BResId > 0)
        self.RemoveSubPart(self.BResId);
      if (self.BPolId > 0)
        self.RemoveSubPart(self.BPolId);
      if (self.BResTextId > 0)
        self.RemoveSubPart(self.BResTextId);
      if (self.BPolTextId > 0)
        self.RemoveSubPart(self.BPolTextId);
      if (self.BAIId > 0)
        self.RemoveSubPart(self.BAIId);
      if (self.BAITextID > 0)
        self.RemoveSubPart(self.BAITextID);
      if (self.e1id > 0)
        self.RemoveSubPart(self.e1id);
      if (self.e1textid > 0)
        self.RemoveSubPart(self.e1textid);
      if (self.e2id > 0)
        self.RemoveSubPart(self.e2id);
      if (self.e2textid > 0)
        self.RemoveSubPart(self.e2textid);
      if (self.e3id > 0)
        self.RemoveSubPart(self.e3id);
      if (self.e3textid > 0)
        self.RemoveSubPart(self.e3textid);
      if (self.e4id > 0)
        self.RemoveSubPart(self.e4id);
      if (self.e4textid > 0)
        self.RemoveSubPart(self.e4textid);
      if (self.e5id > 0)
        self.RemoveSubPart(self.e5id);
      if (self.e5textid > 0)
        self.RemoveSubPart(self.e5textid);
      if (self.e6id > 0)
        self.RemoveSubPart(self.e6id);
      if (self.e6textid > 0)
        self.RemoveSubPart(self.e6textid);
      if (self.e7id > 0)
        self.RemoveSubPart(self.e7id);
      if (self.e7textid > 0)
        self.RemoveSubPart(self.e7textid);
      if (self.e8id > 0)
        self.RemoveSubPart(self.e8id);
      if (self.e8textid > 0)
        self.RemoveSubPart(self.e8textid);
      if (self.e9id > 0)
        self.RemoveSubPart(self.e9id);
      if (self.e9textid > 0)
        self.RemoveSubPart(self.e9textid);
      if (self.e10id > 0)
        self.RemoveSubPart(self.e10id);
      if (self.e10textid > 0)
        self.RemoveSubPart(self.e10textid);
      if (self.e11id > 0)
        self.RemoveSubPart(self.e11id);
      if (self.e11textid > 0)
        self.RemoveSubPart(self.e11textid);
      if (self.e12id > 0)
        self.RemoveSubPart(self.e12id);
      if (self.e12textid > 0)
        self.RemoveSubPart(self.e12textid);
      if (self.e13id > 0)
        self.RemoveSubPart(self.e13id);
      if (self.e13textid > 0)
        self.RemoveSubPart(self.e13textid);
      if (self.PbemId > 0)
        self.RemoveSubPart(self.PbemId);
      if (self.PbemTextId > 0)
        self.RemoveSubPart(self.PbemTextId);
      if (self.NatIconId > 0)
        self.RemoveSubPart(self.NatIconId);
      if (self.NatIconPic > 0)
        self.RemoveSubPart(self.NatIconPic);
      if (self.RoundelId > 0)
        self.RemoveSubPart(self.RoundelId);
      if (self.RoundelPic > 0)
        self.RemoveSubPart(self.RoundelPic);
      if (self.HighId > 0)
        self.RemoveSubPart(self.HighId);
      if (self.LowId > 0)
        self.RemoveSubPart(self.LowId);
      if (self.regimeNr > -1)
      {
        self.ss = "Click to change the name of the regime";
        let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BNameId = self.AddSubPart( tsubpart1, 370, 220, 32, 16, 1);
        let mut tsubpart2: SubPartClass =  TextPartClass::new("Name: " + self.game.Data.RegimeObj[self.regimeNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BNameTextId = self.AddSubPart( tsubpart2, 410, 219, 200, 20, 0);
        self.ss = "Click to change the of: Color the counters and backgrounds of this regime";
        let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BRedId = self.AddSubPart( tsubpart3, 370, 50, 32, 16, 1);
        let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawHistoryForce(self.regimeNr, -1, -1));
        self.BRedTextId = self.AddSubPart( tsubpart4, 410, 49, 38, 38, 0);
        self.ss = "Click to change the of: Color the sprite and text on the counters, and over backgrounds";
        let mut tsubpart5: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BRed2Id = self.AddSubPart( tsubpart5, 370, 120, 32, 16, 1);
        if (self.game.Data.SFTypeCounter > -1)
        {
          let mut tsubpart6: SubPartClass =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawHistoryForce(self.regimeNr, 50, 0));
          self.BRed2TextId = self.AddSubPart( tsubpart6, 410, 119, 200, 20, 0);
        }
        self.ss = "Click to set basemorale for regime. 100 means the basemorale of a people is not modified. 50 means basemorale of a people will be 50% lower.";
        let mut tsubpart7: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BMorId = self.AddSubPart( tsubpart7, 370, 180, 32, 16, 1);
        let mut tsubpart8: SubPartClass =  TextPartClass::new("BaseMorale: " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].BaseMorale), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BMorTextId = self.AddSubPart( tsubpart8, 410, 179, 200, 20, 0);
        self.ss = "Click to set the political points this regime starts with";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BResId = self.AddSubPart( tsubpart8, 370, 200, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("Pol.Pts: " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].ResPts), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BResTextId = self.AddSubPart( tsubpart8, 410, 199, 200, 20, 0);
        self.ss = "Click to set if this regime is an AI as default or not";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BAIId = self.AddSubPart( tsubpart8, 370, 240, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("is AI: " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].AI), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BAITextID = self.AddSubPart( tsubpart8, 410, 239, 200, 20, 0);
        self.ss = "Click to set if this regime is sleeping. (sleeping means it wont be able to play until an event wakes it)";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.e5id = self.AddSubPart( tsubpart8, 670, 220, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("Sleeping: " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].Sleep), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.e5textid = self.AddSubPart( tsubpart8, 710, 219, 200, 20, 0);
        self.ss = "Click to set the PBEM++ Player for this regime. 0=auto set, 1=player 1, 2=player 2";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.PbemId = self.AddSubPart( tsubpart8, 670, 240, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("PBEM++ player: " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].PbemPlayer), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.PbemTextId = self.AddSubPart( tsubpart8, 710, 239, 200, 20, 0);
        self.ss = "Set to true if regime uses Alternative actioncard pictures";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.altid = self.AddSubPart( tsubpart8, 670, 260, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("Alt.ActionCardPics: " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].UseAlternateActionCardPics), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.alttextid = self.AddSubPart( tsubpart8, 710, 259, 200, 20, 0);
        self.ss = "Click to set all subformations of this regime to its current people!";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.e9id = self.AddSubPart( tsubpart8, 670, 170, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("Set All SF Ppl", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.e9textid = self.AddSubPart( tsubpart8, 710, 169, 200, 20, 0);
        self.ss = "Click to remove a regime";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
        self.BRemoveregimeId = self.AddSubPart( tsubpart8, 610, 90, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("Remove ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BRemoveregimeTextId = self.AddSubPart( tsubpart8, 650, 89, 200, 20, 0);
        self.ss = "Click to change Lib";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.clid = self.AddSubPart( tsubpart8, 850, 90, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("Set Library", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.cltextid = self.AddSubPart( tsubpart8, 890, 89, 200, 20, 0);
        if (self.regimeNr < self.game.Data.RegimeCounter)
        {
          tsubpart8 =  ButtonPartClass::new(self.game.BUTTONDOWN, tDescript: "Move down list");
          self.HighId = self.AddSubPart( tsubpart8, 10, 330, 32, 16, 1);
        }
        if (self.regimeNr > 0)
        {
          tsubpart8 =  ButtonPartClass::new(self.game.BUTTONUP, tDescript: "Move Up List");
          self.LowId = self.AddSubPart( tsubpart8, 50, 330, 32, 16, 1);
        }
        self.ss = "Click to use this regime for drawing on the map";
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONDRAW, tDescript: self.ss);
        self.BDrawId = self.AddSubPart( tsubpart8, 610, 110, 32, 16, 1);
        tsubpart8 =  TextPartClass::new("Select as pencil", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BDrawTextId = self.AddSubPart( tsubpart8, 650, 109, 200, 20, 0);
        self.OptionsListObj = ListClass::new();
        self.OptionsListObj.add("Statistics", 0);
        self.OptionsListObj.add("RegimeSlots", 1);
        self.OptionsListObj.add("Diplomatic Relations", 2);
        ListClass optionsListObj = self.OptionsListObj;
        let mut tabSheetNr: i32 = self.TabSheetNr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart8 =  new ListSubPartClass(optionsListObj, 5, 300, tabSheetNr, game, tHeader: "Property Sheets", tbackbitmap: ( local1), bbx: 370, bby: 262, overruleFont: ( local2));
        self.OptionsListId = self.AddSubPart( tsubpart8, 370, 262, 300, 128, 0);
      }
      self.maketabsheet();
    }

     void maketabsheet()
    {
      if (self.B1Id > 0)
        self.RemoveSubPart(self.B1Id);
      if (self.B1TextId > 0)
        self.RemoveSubPart(self.B1TextId);
      if (self.B2Id > 0)
        self.RemoveSubPart(self.B2Id);
      if (self.B2TextId > 0)
        self.RemoveSubPart(self.B2TextId);
      if (self.b3Id > 0)
        self.RemoveSubPart(self.b3Id);
      if (self.B3TextId > 0)
        self.RemoveSubPart(self.B3TextId);
      if (self.b4Id > 0)
        self.RemoveSubPart(self.b4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      if (self.b5Id > 0)
        self.RemoveSubPart(self.b5Id);
      if (self.B5TextId > 0)
        self.RemoveSubPart(self.B5TextId);
      if (self.e8id > 0)
        self.RemoveSubPart(self.e8id);
      if (self.e8textid > 0)
        self.RemoveSubPart(self.e8textid);
      if (self.stringListId > 0)
        self.RemoveSubPart(self.stringListId);
      if (self.ResListId > 0)
        self.RemoveSubPart(self.ResListId);
      if (self.BSymbolId > 0)
        self.RemoveSubPart(self.BSymbolId);
      if (self.BChangeSymbolId > 0)
        self.RemoveSubPart(self.BChangeSymbolId);
      if (self.NatIconId > 0)
        self.RemoveSubPart(self.NatIconId);
      if (self.NatIconPic > 0)
        self.RemoveSubPart(self.NatIconPic);
      if (self.RoundelId > 0)
        self.RemoveSubPart(self.RoundelId);
      if (self.RoundelPic > 0)
        self.RemoveSubPart(self.RoundelPic);
      if (self.e10id > 0)
        self.RemoveSubPart(self.e10id);
      if (self.e10textid > 0)
        self.RemoveSubPart(self.e10textid);
      if (self.e11id > 0)
        self.RemoveSubPart(self.e11id);
      if (self.e11textid > 0)
        self.RemoveSubPart(self.e11textid);
      if (self.e12id > 0)
        self.RemoveSubPart(self.e12id);
      if (self.e12textid > 0)
        self.RemoveSubPart(self.e12textid);
      if (self.e13id > 0)
        self.RemoveSubPart(self.e13id);
      if (self.e13textid > 0)
        self.RemoveSubPart(self.e13textid);
      if (self.dipListId > 0)
        self.RemoveSubPart(self.dipListId);
      if (!(self.regimeNr > -1 & self.TabSheetNr > -1))
        return;
      if (self.TabSheetNr == 0)
        self.maketabsheetnr0();
      if (self.TabSheetNr == 1)
        self.maketabsheetnr1();
      if (self.TabSheetNr != 2)
        return;
      self.maketabsheetnr4();
    }

     void maketabsheetnr0()
    {
      self.ss = "Click to select which people rule this regime. (is important for people basemorale and production)";
      name: String = self.game.Data.PeopleObj[self.game.Data.RegimeObj[self.regimeNr].People].Name;
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.B1Id = self.AddSubPart( tsubpart1, 10, 380, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("People: " + name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B1TextId = self.AddSubPart( tsubpart2, 50, 379, 400, 20, 0);
      self.ss = "Click to change the HQ sprite";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.Data.RegimeObj[self.regimeNr].HQSpriteNr, tDescript: self.ss);
      self.BSymbolId = self.AddSubPart( tsubpart3, 10, 410, 31, 31, 0);
      let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.BChangeSymbolId = self.AddSubPart( tsubpart4, 50, 410, 32, 16, 1);
      self.ss = "Click to select if this HQ sprite should have its colours adjusted for the regimes color.";
      let mut tsubpart5: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.e8id = self.AddSubPart( tsubpart5, 10, 450, 32, 16, 1);
      let mut tsubpart6: SubPartClass =  TextPartClass::new("HQSymbolOverrule: " + Strings.Trim(Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].HQSpriteOverrule)), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.e8textid = self.AddSubPart( tsubpart6, 50, 449, 400, 20, 0);
      self.ss = "Click to change the National Icon of this regime";
      let mut tsubpart7: SubPartClass =  ButtonPartClass::new(self.game.Data.RegimeObj[self.regimeNr].NationalIconSprite, tDescript: self.ss);
      self.NatIconPic = self.AddSubPart( tsubpart7, 10, 490, 10, 10, 0);
      let mut tsubpart8: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.NatIconId = self.AddSubPart( tsubpart8, 50, 490, 32, 16, 1);
      self.ss = "Click to set UberRegime for this Regime.";
      let mut tsubpart9: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.e12id = self.AddSubPart( tsubpart9, 10, 580, 32, 16, 1);
      SubPartClass tsubpart10;
      if (self.game.Data.RegimeObj[self.regimeNr].UberRegime == -1)
      {
        tsubpart10 =  TextPartClass::new("UberRegime: -1 (none)", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.e12textid = self.AddSubPart( tsubpart10, 50, 579, 400, 20, 0);
      }
      else
      {
        let mut tsubpart11: SubPartClass =  TextPartClass::new("UberRegime: " + self.game.Data.RegimeObj[self.game.Data.RegimeObj[self.regimeNr].UberRegime].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.e12textid = self.AddSubPart( tsubpart11, 50, 579, 400, 20, 0);
      }
      self.ss = "Click to set if SFType sprites should be mirrored for this regime";
      tsubpart10 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.e13id = self.AddSubPart( tsubpart10, 10, 610, 32, 16, 1);
      tsubpart10 =  TextPartClass::new("Mirror: " + Strings.Trim(Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].Mirror)), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.e13textid = self.AddSubPart( tsubpart10, 50, 609, 400, 20, 0);
      self.ss = "Click to change the Roundel Icon of this regime";
      tsubpart10 =  ButtonPartClass::new(self.game.Data.RegimeObj[self.regimeNr].RoundelIconSprite, tDescript: self.ss, tResizeX: 20, tresizeY: 20);
      self.RoundelPic = self.AddSubPart( tsubpart10, 10, 630, 10, 10, 0);
      tsubpart10 =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.RoundelId = self.AddSubPart( tsubpart10, 50, 630, 32, 16, 1);
    }

     void maketabsheetnr1()
    {
      self.stringListObj = ListClass::new();
      let mut index: i32 = 0;
      do
      {
        self.stringListObj.add(Conversion.Str( index) + ") " + self.game.Data.RegimeSlotName[index] + " = " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].RegimeSlot[index]), index);
        index += 1;
      }
      while (index <= 499);
      if (self.DetailNr > self.game.Data.StringCounter)
        self.DetailNr = -1;
      ListClass stringListObj = self.stringListObj;
      let mut detailNr: i32 = self.DetailNr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(stringListObj, 12, 300, detailNr, game, tHeader: "Regimeslots", tbackbitmap: ( local1), bbx: 10, bby: 400, overruleFont: ( local2));
      self.stringListId = self.AddSubPart( tsubpart, 10, 400, 300, 240, 0);
      if (self.DetailNr <= -1)
        return;
      self.maketabsheetnr1b();
    }

     void maketabsheetnr1b()
    {
      self.ss = "Click to change value of regimeslots. Regimeslots can be used in events. Their names can be set in settings";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.B2Id = self.AddSubPart( tsubpart1, 350, 400, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Change Value", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B2TextId = self.AddSubPart( tsubpart2, 390, 399, 400, 20, 0);
    }

     void maketabsheetnr3()
    {
      self.ResListObj = ListClass::new();
      if (self.game.Data.ResearchCounter <= -1)
        return;
      let mut researchCounter: i32 = self.game.Data.ResearchCounter;
      for (let mut index: i32 = 0; index <= researchCounter; index += 1)
        self.ResListObj.add(Conversion.Str( index) + ") " + self.game.Data.ResearchObj[index].Name + " = " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].ResField[index]), index);
      if (self.DetailNr > self.game.Data.ResearchCounter)
        self.DetailNr = -1;
      ListClass resListObj = self.ResListObj;
      let mut detailNr: i32 = self.DetailNr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(resListObj, 12, 300, detailNr, game, tHeader: "Regimes Research", tbackbitmap: ( local1), bbx: 10, bby: 400, overruleFont: ( local2));
      self.ResListId = self.AddSubPart( tsubpart, 10, 400, 300, 240, 0);
      if (self.DetailNr <= -1)
        return;
      self.maketabsheetnr3b();
    }

     void maketabsheetnr3b()
    {
      self.ss = "Click to enable or disable a researchfield for this regime";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.b3Id = self.AddSubPart( tsubpart1, 350, 400, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Change Value", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B3TextId = self.AddSubPart( tsubpart2, 390, 399, 400, 20, 0);
      self.ss = "Click here to set all research up to lvl X on.. and above off";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.b5Id = self.AddSubPart( tsubpart3, 350, 450, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("Set research to lvl", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B5TextId = self.AddSubPart( tsubpart4, 390, 449, 400, 20, 0);
    }

     void maketabsheetnr4()
    {
      self.dipListObj = ListClass::new();
      if (self.game.Data.RegimeCounter <= -1)
        return;
      let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
      for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
        self.dipListObj.add(Conversion.Str( index) + ") " + self.game.Data.RegimeObj[index].Name + " = " + Conversion.Str( self.game.Data.RegimeObj[self.regimeNr].RegimeRel[index]), index);
      if (self.DetailNr > self.game.Data.RegimeCounter)
        self.DetailNr = -1;
      ListClass dipListObj = self.dipListObj;
      let mut detailNr: i32 = self.DetailNr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(dipListObj, 12, 300, detailNr, game, tHeader: "Dip", tbackbitmap: ( local1), bbx: 10, bby: 400, overruleFont: ( local2));
      self.dipListId = self.AddSubPart( tsubpart, 10, 400, 300, 240, 0);
      if (self.DetailNr <= -1)
        return;
      self.maketabsheetnr4b();
    }

     void maketabsheetnr4b()
    {
      self.ss = "Click to change diplomatic relation of selected regime with this regime";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.b4Id = self.AddSubPart( tsubpart1, 350, 400, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Change Value", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.B4TextId = self.AddSubPart( tsubpart2, 390, 399, 400, 20, 0);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.LibListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.LibNr = num2;
                self.regimeNr = -1;
                self.MakeregimeListGUI(self.regimeNr);
              }
              else if (num2 == -2)
              {
                self.LibNr = -1;
                self.regimeNr = -1;
                self.MakeregimeListGUI(self.regimeNr);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.regimeListId)
            {
              let mut num3: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                self.regimeNr = num3;
                self.MakeregimeTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BAddregimeId)
            {
              self.game.Data.AddRegime();
              self.MakeregimeListGUI(self.regimeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BNameId)
            {
              self.game.Data.RegimeObj[self.regimeNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              self.MakeregimeListGUI(self.regimeNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BMorId)
            {
              let mut num4: i32 =  Math.Round(Conversion.Int(Conversion.Val(Conversions.ToString(Conversion.Val(Interaction.InputBox("Give new base morale please.", "Shadow Empire : Planetary Conquest"))))));
              if (num4 > -1 & num4 < 300)
              {
                self.game.Data.RegimeObj[self.regimeNr].BaseMorale = num4;
                self.MakeregimeListGUI(self.regimeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut num5: i32 =  Interaction.MsgBox( "Wrong input. between 0-300 please. Cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
            }
            else if (num1 == self.BResId)
            {
              let mut num6: i32 =  Math.Round(Conversion.Int(Conversion.Val(Conversions.ToString(Conversion.Val(Interaction.InputBox("Give new political pts please.", "Shadow Empire : Planetary Conquest"))))));
              if (num6 > -1 & num6 < 9999)
              {
                self.game.Data.RegimeObj[self.regimeNr].ResPts = num6;
                self.MakeregimeListGUI(self.regimeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut num7: i32 =  Interaction.MsgBox( "Wrong input. between 0-9999 please. Cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
            }
            else
            {
              if (num1 == self.BAIId)
              {
                self.game.Data.RegimeObj[self.regimeNr].AI = !self.game.Data.RegimeObj[self.regimeNr].AI;
                self.MakeregimeListGUI(self.regimeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.altid)
              {
                self.game.Data.RegimeObj[self.regimeNr].UseAlternateActionCardPics = !self.game.Data.RegimeObj[self.regimeNr].UseAlternateActionCardPics;
                self.MakeregimeListGUI(self.regimeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.e5id)
              {
                self.game.Data.RegimeObj[self.regimeNr].Sleep = !self.game.Data.RegimeObj[self.regimeNr].Sleep;
                self.MakeregimeListGUI(self.regimeNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.PbemId)
              {
                let mut num8: i32 =  Math.Round(Conversion.Int(Conversion.Val(Conversions.ToString(Conversion.Val(Interaction.InputBox("Give new PBEM++ player please. 0=auto/no-overrule. 1=player 1, 2=player 2. Remember PBEM++ games always should have 2 human players! no more no less.", "Shadow Empire : Planetary Conquest"))))));
                if (num8 >= 0 & num8 < 3)
                {
                  self.game.Data.RegimeObj[self.regimeNr].PbemPlayer = num8;
                  self.MakeregimeListGUI(self.regimeNr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num9: i32 =  Interaction.MsgBox( "Wrong input. between 0-2 please. Cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                if (num1 == self.clid)
                {
                  Form3::new( self.formref).Initialize(self.game.Data, 97, self.regimeNr);
                  self.MakeregimeListGUI(self.regimeNr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.BRedId)
                {
                  ColorDialog colorDialog = ColorDialog::new();
                  colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[self.regimeNr].Red, self.game.Data.RegimeObj[self.regimeNr].Green, self.game.Data.RegimeObj[self.regimeNr].Blue);
                  if (colorDialog.ShowDialog() == DialogResult.OK)
                  {
                    RegimeClass regimeClass1 = self.game.Data.RegimeObj[self.regimeNr];
                    color: Color = colorDialog.Color;
                    let mut r: i32 =  color.R;
                    regimeClass1.Red = r;
                    RegimeClass regimeClass2 = self.game.Data.RegimeObj[self.regimeNr];
                    color = colorDialog.Color;
                    let mut g: i32 =  color.G;
                    regimeClass2.Green = g;
                    RegimeClass regimeClass3 = self.game.Data.RegimeObj[self.regimeNr];
                    color = colorDialog.Color;
                    let mut b1: i32 =  color.B;
                    regimeClass3.Blue = b1;
                    self.game.Data.RegimeObj[self.regimeNr].HexBack = (Bitmap) null;
                    self.game.Data.RegimeObj[self.regimeNr].TempCounter = (Bitmap) null;
                  }
                  self.MakeregimeListGUI(self.regimeNr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.BRed3Id)
                {
                  ColorDialog colorDialog = ColorDialog::new();
                  colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[self.regimeNr].Red3, self.game.Data.RegimeObj[self.regimeNr].Green3, self.game.Data.RegimeObj[self.regimeNr].Blue3);
                  if (colorDialog.ShowDialog() == DialogResult.OK)
                  {
                    RegimeClass regimeClass4 = self.game.Data.RegimeObj[self.regimeNr];
                    color: Color = colorDialog.Color;
                    let mut r: i32 =  color.R;
                    regimeClass4.Red3 = r;
                    RegimeClass regimeClass5 = self.game.Data.RegimeObj[self.regimeNr];
                    color = colorDialog.Color;
                    let mut g: i32 =  color.G;
                    regimeClass5.Green3 = g;
                    RegimeClass regimeClass6 = self.game.Data.RegimeObj[self.regimeNr];
                    color = colorDialog.Color;
                    let mut b2: i32 =  color.B;
                    regimeClass6.Blue3 = b2;
                  }
                  self.MakeregimeListGUI(self.regimeNr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.BRed4Id)
                {
                  ColorDialog colorDialog = ColorDialog::new();
                  colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[self.regimeNr].Red4, self.game.Data.RegimeObj[self.regimeNr].Green4, self.game.Data.RegimeObj[self.regimeNr].Blue4);
                  if (colorDialog.ShowDialog() == DialogResult.OK)
                  {
                    RegimeClass regimeClass7 = self.game.Data.RegimeObj[self.regimeNr];
                    color: Color = colorDialog.Color;
                    let mut r: i32 =  color.R;
                    regimeClass7.Red4 = r;
                    RegimeClass regimeClass8 = self.game.Data.RegimeObj[self.regimeNr];
                    color = colorDialog.Color;
                    let mut g: i32 =  color.G;
                    regimeClass8.Green4 = g;
                    RegimeClass regimeClass9 = self.game.Data.RegimeObj[self.regimeNr];
                    color = colorDialog.Color;
                    let mut b3: i32 =  color.B;
                    regimeClass9.Blue4 = b3;
                  }
                  self.MakeregimeListGUI(self.regimeNr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.BRed2Id)
                {
                  ColorDialog colorDialog = ColorDialog::new();
                  colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[self.regimeNr].Red2, self.game.Data.RegimeObj[self.regimeNr].Green2, self.game.Data.RegimeObj[self.regimeNr].Blue2);
                  if (colorDialog.ShowDialog() == DialogResult.OK)
                  {
                    RegimeClass regimeClass10 = self.game.Data.RegimeObj[self.regimeNr];
                    color: Color = colorDialog.Color;
                    let mut r: i32 =  color.R;
                    regimeClass10.Red2 = r;
                    RegimeClass regimeClass11 = self.game.Data.RegimeObj[self.regimeNr];
                    color = colorDialog.Color;
                    let mut g: i32 =  color.G;
                    regimeClass11.Green2 = g;
                    RegimeClass regimeClass12 = self.game.Data.RegimeObj[self.regimeNr];
                    color = colorDialog.Color;
                    let mut b4: i32 =  color.B;
                    regimeClass12.Blue2 = b4;
                    self.game.Data.RegimeObj[self.regimeNr].HexBack = (Bitmap) null;
                    self.game.Data.RegimeObj[self.regimeNr].TempCounter = (Bitmap) null;
                  }
                  self.MakeregimeListGUI(self.regimeNr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.BRemoveregimeId)
                {
                  self.game.EditObj.UnitSelected = -1;
                  self.game.Data.RemoveRegime(self.regimeNr);
                  self.MakeregimeListGUI(-1);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.HighId)
                {
                  self.game.EditObj.UnitSelected = -1;
                  self.game.Data.MoveRegimeHigher(self.regimeNr);
                  this += 1.regimeNr;
                  self.MakeregimeListGUI(self.regimeNr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.LowId)
                {
                  self.game.EditObj.UnitSelected = -1;
                  self.game.Data.MoveRegimeLower(self.regimeNr);
                  --self.regimeNr;
                  self.MakeregimeListGUI(self.regimeNr);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.BDrawId)
                {
                  self.game.EditObj.PencilType = 3;
                  self.game.EditObj.PencilData1 = self.regimeNr;
                  windowReturnClass.AddCommand(4, 13);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.OptionsListId)
                {
                  let mut num10: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (num10 > -1)
                  {
                    self.TabSheetNr = num10;
                    self.maketabsheet();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.B1Id)
                {
                  Form3::new( self.formref).Initialize(self.game.Data, 3, self.regimeNr);
                  self.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.e11id)
                {
                  Form3::new( self.formref).Initialize(self.game.Data, 55, self.regimeNr);
                  self.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.e12id)
                {
                  Form3::new( self.formref).Initialize(self.game.Data, 57, self.regimeNr);
                  self.maketabsheet();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.stringListId)
                {
                  let mut num11: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (num11 > -1)
                  {
                    self.DetailNr = num11;
                    self.maketabsheet();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.ResListId)
                {
                  let mut num12: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (num12 > -1)
                  {
                    self.DetailNr = num12;
                    self.maketabsheet();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.dipListId)
                {
                  let mut num13: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (num13 > -1)
                  {
                    self.DetailNr = num13;
                    self.maketabsheet();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.B2Id)
                {
                  let mut num14: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new people #, please.", "Shadow Empire : Planetary Conquest")));
                  if (num14 >= -1 & num14 <= 999999)
                  {
                    self.game.Data.RegimeObj[self.regimeNr].RegimeSlot[self.DetailNr] = num14;
                    self.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  let mut num15: i32 =  Interaction.MsgBox( "Wrong input. Cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else if (num1 == self.e10id)
                {
                  let mut num16: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give extra graphic #, please.", "Shadow Empire : Planetary Conquest")));
                  if (num16 >= -1 & num16 <= 999999)
                  {
                    self.game.Data.RegimeObj[self.regimeNr].ExtraGraphicUse = num16;
                    self.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  let mut num17: i32 =  Interaction.MsgBox( "Wrong input. Cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  if (num1 == self.b3Id)
                  {
                    self.game.Data.RegimeObj[self.regimeNr].ResField[self.DetailNr] = !self.game.Data.RegimeObj[self.regimeNr].ResField[self.DetailNr];
                    self.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == self.b5Id)
                  {
                    let mut num18: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give research level for this regime...", "Shadow Empire : Planetary Conquest")));
                    if (num18 >= -1 & num18 <= 999999)
                    {
                      let mut researchCounter: i32 = self.game.Data.ResearchCounter;
                      for (let mut index2: i32 = 0; index2 <= researchCounter; index2 += 1)
                      {
                        if (self.game.Data.ResearchObj[index2].TechLevel <= num18 & self.game.Data.ResearchObj[index2].TechLevel > 0)
                          self.game.Data.RegimeObj[self.regimeNr].ResField[index2] = true;
                        else
                          self.game.Data.RegimeObj[self.regimeNr].ResField[index2] = false;
                      }
                      self.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    let mut num19: i32 =  Interaction.MsgBox( "Wrong input. Cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else if (num1 == self.b4Id)
                  {
                    if (self.DetailNr != self.regimeNr)
                    {
                      let mut num20: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new relation please. 0=war, 1=peace, 2=allied ", "Shadow Empire : Planetary Conquest")));
                      if (num20 < 0 | num20 > 2)
                      {
                        let mut num21: i32 =  Interaction.MsgBox( "Beep. Not allowed. ", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      else
                      {
                        self.game.Data.RegimeObj[self.regimeNr].RegimeRel[self.DetailNr] = num20;
                        self.game.Data.RegimeObj[self.DetailNr].RegimeRel[self.regimeNr] = num20;
                        self.maketabsheet();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                    }
                    else
                    {
                      let mut num22: i32 =  Interaction.MsgBox( "You cannot change relation with your self", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                  }
                  else
                  {
                    if (num1 == self.BChangeSymbolId)
                    {
                      filename: String = self.game.HandyFunctionsObj.LoadSomething("Png (*.png)|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of HQ Symbol Sprite:", self.game.AppPath + "graphics\\", true);
                      if (File.Exists(self.game.AppPath + "graphics/" + filename))
                      {
                        self.game.Data.RegimeObj[self.regimeNr].ReplaceHQSprite(filename);
                      }
                      else
                      {
                        let mut num23: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      self.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == self.NatIconId)
                    {
                      filename: String = self.game.HandyFunctionsObj.LoadSomething("Png (*.png)|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of National Identifier of this regime:", self.game.AppPath + "graphics\\", true);
                      if (File.Exists(self.game.AppPath + "graphics/" + filename))
                      {
                        self.game.Data.RegimeObj[self.regimeNr].ReplaceNationalSprite(filename);
                      }
                      else
                      {
                        let mut num24: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      self.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == self.RoundelId)
                    {
                      filename: String = self.game.HandyFunctionsObj.LoadSomething("Png (*.png)|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of National Identifier of this regime:", self.game.AppPath + "graphics\\", true);
                      if (File.Exists(self.game.AppPath + "graphics/" + filename))
                      {
                        self.game.Data.RegimeObj[self.regimeNr].ReplaceRoundelSprite(filename);
                      }
                      else
                      {
                        let mut num25: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      self.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == self.e1id)
                    {
                      self.game.Data.RegimeObj[self.regimeNr].UnitName = Interaction.InputBox("Give new UnitName.", "Shadow Empire : Planetary Conquest");
                      self.MakeregimeListGUI(self.regimeNr);
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == self.e2id)
                    {
                      let mut num26: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new Unit Counter.", "Shadow Empire : Planetary Conquest")));
                      if (num26 > -1)
                      {
                        self.game.Data.RegimeObj[self.regimeNr].UnitNumber = num26;
                        self.MakeregimeListGUI(self.regimeNr);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      let mut num27: i32 =  Interaction.MsgBox( "Invalid input", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      if (num1 == self.e3id)
                      {
                        self.game.Data.RegimeObj[self.regimeNr].HQName = Interaction.InputBox("Give new HQName.", "Shadow Empire : Planetary Conquest");
                        self.MakeregimeListGUI(self.regimeNr);
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == self.e9id)
                      {
                        let mut unitCounter: i32 = self.game.Data.UnitCounter;
                        for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
                        {
                          if (self.game.Data.UnitObj[index3].Regime == self.regimeNr)
                          {
                            let mut sfCount: i32 = self.game.Data.UnitObj[index3].SFCount;
                            for (let mut index4: i32 = 0; index4 <= sfCount; index4 += 1)
                              self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index4]].People = self.game.Data.RegimeObj[self.regimeNr].People;
                          }
                        }
                        let mut num28: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      else if (num1 == self.e4id)
                      {
                        let mut num29: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new HQ Counter.", "Shadow Empire : Planetary Conquest")));
                        if (num29 > -1)
                        {
                          self.game.Data.RegimeObj[self.regimeNr].HQNumber = num29;
                          self.MakeregimeListGUI(self.regimeNr);
                          windowReturnClass.SetFlag(true);
                          return windowReturnClass;
                        }
                        let mut num30: i32 =  Interaction.MsgBox( "Invalid input", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      else
                      {
                        if (num1 == self.e7id)
                        {
                          self.game.Data.RegimeObj[self.regimeNr].DipBlock = !self.game.Data.RegimeObj[self.regimeNr].DipBlock;
                          self.MakeregimeListGUI(self.regimeNr);
                          windowReturnClass.SetFlag(true);
                          return windowReturnClass;
                        }
                        if (num1 == self.e8id)
                        {
                          self.game.Data.RegimeObj[self.regimeNr].HQSpriteOverrule = !self.game.Data.RegimeObj[self.regimeNr].HQSpriteOverrule;
                          self.MakeregimeListGUI(self.regimeNr);
                          windowReturnClass.SetFlag(true);
                          return windowReturnClass;
                        }
                        if (num1 == self.e13id)
                        {
                          self.game.Data.RegimeObj[self.regimeNr].Mirror = !self.game.Data.RegimeObj[self.regimeNr].Mirror;
                          self.MakeregimeListGUI(self.regimeNr);
                          windowReturnClass.SetFlag(true);
                          return windowReturnClass;
                        }
                      }
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
  }
}
