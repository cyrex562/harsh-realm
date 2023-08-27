// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RoadTypeWindowClass
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

namespace WindowsApplication1
{
  pub class RoadTypeWindowClass : WindowClass
  {
     RoadListId: i32;
     ListClass RoadListObj;
     BAddRoadId: i32;
     BAddRoadTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     B7Id: i32;
     B7TextId: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     BRemoveRoadId: i32;
     BRemoveRoadTextId: i32;
     BDrawId: i32;
     BDrawTextId: i32;
     ListClass BasicListObj;
     BasicListId: i32;
     b1id: i32;
     b1textid: i32;
     BBasicSpriteId: i32;
     BChangeBasicSpriteId: i32;
     ListClass BasicList2Obj;
     BasicList2Id: i32;
     ChangeMvId: i32;
     int[] Bitemid;
     int[] bitemtextid;
     BEP: i32;
     BEPText: i32;
     txt1: i32;
     bthick: i32;
     bthicktext: i32;
     bother: i32;
     bothertext: i32;
     BridgeId: i32;
     BridgeTextId: i32;
     ListClass BridgeListObj;
     BridgeListId: i32;
     BBridgeSpriteId: i32;
     BChangeBridgeSpriteId: i32;
     BChangeBridgeSpriteId2: i32;
     RoadNr: i32;
     TabSheetNr: i32;
     DetailNr: i32;
     BridgeNr: i32;
     ss: String;

    pub RoadTypeWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Road Types")
    {
      self.Bitemid = new int[5];
      self.bitemtextid = new int[5];
      self.RoadNr = -1;
      self.TabSheetNr = -1;
      self.DetailNr = -1;
      self.BridgeNr = -1;
      self.MakeRoadListGUI(-1);
    }

    pub fn DoRefresh() => self.MakeRoadTypeItemGUI();

     void MakeRoadListGUI(tRoadnr: i32)
    {
      if (self.RoadListId > 0)
        self.RemoveSubPart(self.RoadListId);
      SubPartClass tsubpart;
      if (self.game.Data.RoadTypeCounter > -1)
      {
        self.RoadListObj = ListClass::new();
        let mut roadTypeCounter: i32 = self.game.Data.RoadTypeCounter;
        for (let mut index: i32 = 0; index <= roadTypeCounter; index += 1)
          self.RoadListObj.add(Conversion.Str( index) + ") " + self.game.Data.RoadTypeObj[index].Name, index);
        ListClass roadListObj = self.RoadListObj;
        let mut tlistselect: i32 = tRoadnr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart =  new ListSubPartClass(roadListObj, 9, 200, tlistselect, game, tHeader: "Road Types", tbackbitmap: ( local1), bbx: 10, bby: 50, overruleFont: ( local2));
        self.RoadListId = self.AddSubPart( tsubpart, 10, 50, 200, 192, 0);
        self.RoadNr = tRoadnr;
        self.MakeRoadTypeItemGUI();
      }
      else
      {
        self.RoadNr = tRoadnr;
        self.MakeRoadTypeItemGUI();
      }
      if (self.BAddRoadId > 0)
        self.RemoveSubPart(self.BAddRoadId);
      if (self.BAddRoadTextId > 0)
        self.RemoveSubPart(self.BAddRoadTextId);
      self.ss = "Click to add a new RoadType";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
        self.BAddRoadId = self.AddSubPart( tsubpart, 10, 270, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      tsubpart =  TextPartClass::new("Add Road Type", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.BAddRoadTextId = self.AddSubPart( tsubpart, 50, 269, 300, 20, 0);
    }

     void MakeRoadTypeItemGUI()
    {
      if (self.BNameId > 0)
        self.RemoveSubPart(self.BNameId);
      if (self.BNameTextId > 0)
        self.RemoveSubPart(self.BNameTextId);
      if (self.BRemoveRoadId > 0)
        self.RemoveSubPart(self.BRemoveRoadId);
      if (self.BRemoveRoadTextId > 0)
        self.RemoveSubPart(self.BRemoveRoadTextId);
      if (self.BDrawId > 0)
        self.RemoveSubPart(self.BDrawId);
      if (self.BDrawTextId > 0)
        self.RemoveSubPart(self.BDrawTextId);
      if (self.OptionsListId > 0)
        self.RemoveSubPart(self.OptionsListId);
      let mut index: i32 = 0;
      do
      {
        if (self.Bitemid[index] > 0)
          self.RemoveSubPart(self.Bitemid[index]);
        if (self.bitemtextid[index] > 0)
          self.RemoveSubPart(self.bitemtextid[index]);
        index += 1;
      }
      while (index <= 4);
      if (self.BasicListId > 0)
        self.RemoveSubPart(self.BasicListId);
      if (self.BBasicSpriteId > 0)
        self.RemoveSubPart(self.BBasicSpriteId);
      if (self.BChangeBasicSpriteId > 0)
        self.RemoveSubPart(self.BChangeBasicSpriteId);
      if (self.BridgeListId > 0)
        self.RemoveSubPart(self.BridgeListId);
      if (self.BBridgeSpriteId > 0)
        BitmapStore.RemoveBitmapNr(self.BBridgeSpriteId);
      if (self.BChangeBridgeSpriteId > 0)
        self.RemoveSubPart(self.BChangeBridgeSpriteId);
      if (self.BChangeBridgeSpriteId2 > 0)
        self.RemoveSubPart(self.BChangeBridgeSpriteId2);
      if (self.BridgeId > 0)
        self.RemoveSubPart(self.BridgeId);
      if (self.BridgeTextId > 0)
        self.RemoveSubPart(self.BridgeTextId);
      if (self.BasicList2Id > 0)
        self.RemoveSubPart(self.BasicList2Id);
      if (self.ChangeMvId > 0)
        self.RemoveSubPart(self.ChangeMvId);
      if (self.BEP > 0)
        self.RemoveSubPart(self.BEP);
      if (self.BEPText > 0)
        self.RemoveSubPart(self.BEPText);
      if (self.B2Id > 0)
        self.RemoveSubPart(self.B2Id);
      if (self.B2TextId > 0)
        self.RemoveSubPart(self.B2TextId);
      if (self.B3Id > 0)
        self.RemoveSubPart(self.B3Id);
      if (self.B3TextId > 0)
        self.RemoveSubPart(self.B3TextId);
      if (self.B6Id > 0)
        self.RemoveSubPart(self.B6Id);
      if (self.B6TextId > 0)
        self.RemoveSubPart(self.B6TextId);
      if (self.B7Id > 0)
        self.RemoveSubPart(self.B7Id);
      if (self.B7TextId > 0)
        self.RemoveSubPart(self.B7TextId);
      if (self.bthick > 0)
        self.RemoveSubPart(self.bthick);
      if (self.bthicktext > 0)
        self.RemoveSubPart(self.bthicktext);
      if (self.bother > 0)
        self.RemoveSubPart(self.bother);
      if (self.bothertext > 0)
        self.RemoveSubPart(self.bothertext);
      if (self.RoadNr > -1)
      {
        self.ss = "Click to change the name of this roadtype";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.BNameId = self.AddSubPart( tsubpart, 370, 50, 32, 16, 1);
        }
        let mut tsubpart1: SubPartClass =  TextPartClass::new("Name: " + self.game.Data.RoadTypeObj[self.RoadNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.BNameTextId = self.AddSubPart( tsubpart1, 410, 49, 400, 20, 0);
        self.ss = "Click to set transparent settings (works well in combi with overlay map)";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.B2Id = self.AddSubPart( tsubpart2, 370, 70, 32, 16, 1);
        }
        let mut tsubpart3: SubPartClass =  TextPartClass::new("Transparent: " + Conversion.Str( self.game.Data.RoadTypeObj[self.RoadNr].Transparent), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.B2TextId = self.AddSubPart( tsubpart3, 410, 69, 400, 20, 0);
        self.ss = "Click to remove this roadtype";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
          self.BRemoveRoadId = self.AddSubPart( tsubpart3, 10, 290, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  TextPartClass::new("Remove this RoadType", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
          self.BRemoveRoadTextId = self.AddSubPart( tsubpart3, 50, 289, 200, 20, 0);
        }
        self.ss = "Click to use this road type for drawing on the map";
        tsubpart3 =  ButtonPartClass::new(self.game.BUTTONDRAW, tDescript: self.ss);
        self.BDrawId = self.AddSubPart( tsubpart3, 10, 310, 32, 16, 1);
        tsubpart3 =  TextPartClass::new("Select as pencil", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BDrawTextId = self.AddSubPart( tsubpart3, 50, 309, 200, 20, 0);
        self.OptionsListObj = ListClass::new();
        self.OptionsListObj.add("Sprites", 0);
        self.OptionsListObj.add("Movecost for Road", 1);
        self.OptionsListObj.add("Details", 2);
        ListClass optionsListObj = self.OptionsListObj;
        let mut tabSheetNr: i32 = self.TabSheetNr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart3 =  new ListSubPartClass(optionsListObj, 4, 300, tabSheetNr, game, tHeader: "Property Sheets", tbackbitmap: ( local1), bbx: 370, bby: 140, overruleFont: ( local2));
        self.OptionsListId = self.AddSubPart( tsubpart3, 370, 140, 300, 112, 0);
      }
      self.maketabsheet();
    }

     void maketabsheet()
    {
      if (self.BasicListId > 0)
        self.RemoveSubPart(self.BasicListId);
      if (self.BBasicSpriteId > 0)
        self.RemoveSubPart(self.BBasicSpriteId);
      if (self.BChangeBasicSpriteId > 0)
        self.RemoveSubPart(self.BChangeBasicSpriteId);
      if (self.B3Id > 0)
        self.RemoveSubPart(self.B3Id);
      if (self.B3TextId > 0)
        self.RemoveSubPart(self.B3TextId);
      if (self.BEP > 0)
        self.RemoveSubPart(self.BEP);
      if (self.BEPText > 0)
        self.RemoveSubPart(self.BEPText);
      if (self.b1id > 0)
        self.RemoveSubPart(self.b1id);
      if (self.b1textid > 0)
        self.RemoveSubPart(self.b1textid);
      if (self.txt1 > 0)
        self.RemoveSubPart(self.txt1);
      if (self.B5Id > 0)
        self.RemoveSubPart(self.B5Id);
      if (self.B5TextId > 0)
        self.RemoveSubPart(self.B5TextId);
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      if (self.B6Id > 0)
        self.RemoveSubPart(self.B6Id);
      if (self.B6TextId > 0)
        self.RemoveSubPart(self.B6TextId);
      if (self.B7Id > 0)
        self.RemoveSubPart(self.B7Id);
      if (self.B7TextId > 0)
        self.RemoveSubPart(self.B7TextId);
      if (self.BridgeListId > 0)
        self.RemoveSubPart(self.BridgeListId);
      if (self.BBridgeSpriteId > 0)
        self.RemoveSubPart(self.BBridgeSpriteId);
      if (self.BChangeBridgeSpriteId > 0)
        self.RemoveSubPart(self.BChangeBridgeSpriteId);
      if (self.BridgeId > 0)
        self.RemoveSubPart(self.BridgeId);
      if (self.BridgeTextId > 0)
        self.RemoveSubPart(self.BridgeTextId);
      if (!(self.RoadNr > -1 & self.TabSheetNr > -1))
        return;
      if (self.TabSheetNr == 0)
        self.maketabsheetnr0();
      if (self.TabSheetNr == 1)
        self.maketabsheetnr1();
      if (self.TabSheetNr != 2)
        return;
      self.maketabsheetnr2();
    }

     void maketabsheetnr0()
    {
      if (self.ChangeMvId > 0)
        self.RemoveSubPart(self.ChangeMvId);
      if (self.b1id > 0)
        self.RemoveSubPart(self.b1id);
      if (self.b1textid > 0)
        self.RemoveSubPart(self.b1textid);
      if (self.txt1 > 0)
        self.RemoveSubPart(self.txt1);
      if (self.B5Id > 0)
        self.RemoveSubPart(self.B5Id);
      if (self.B5TextId > 0)
        self.RemoveSubPart(self.B5TextId);
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      self.ss = "";
      txt1: String;
      if (self.game.Data.RoadTypeObj[self.RoadNr].SpecialLayer)
      {
        txt1 = "Change to 6 sprites";
        self.ss = "Click to go back to old 6 sprite mode";
      }
      else
      {
        txt1 = "Change to 64 sprites";
        self.ss = "Click to x64 sprite mode just as with landscape type borders";
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
        self.b1id = self.AddSubPart( tsubpart, 500, 350, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new(txt1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.b1textid = self.AddSubPart( tsubpart1, 550, 349, 400, 20, 0);
      if (self.txt1 > 0)
        self.RemoveSubPart(self.txt1);
      if (!self.game.Data.RoadTypeObj[self.RoadNr].SpecialLayer)
      {
        self.BasicListObj = ListClass::new();
        let mut tdata: i32 = 0;
        do
        {
          self.BasicListObj.add(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteFileName[tdata], tdata);
          tdata += 1;
        }
        while (tdata <= 5);
        ListClass basicListObj = self.BasicListObj;
        let mut detailNr: i32 = self.DetailNr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(basicListObj, 10, 300, detailNr, game, tHeader: "Sprites", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
        self.BasicListId = self.AddSubPart( tsubpart2, 10, 350, 300, 208, 0);
        if (self.DetailNr > 5)
          self.DetailNr = -1;
        if (self.DetailNr <= -1)
          return;
        self.maketabsheetnr0b();
      }
      else
      {
        txt2: String = "Currently using a set of 64 sprites.";
        if (!self.game.Data.RoadTypeObj[self.RoadNr].UseSheet)
        {
          let mut tsubpart3: SubPartClass =  TextPartClass::new(txt2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 250, 20, false, tDescript: self.ss);
          self.txt1 = self.AddSubPart( tsubpart3, 10, 398, 250, 20, 1);
        }
        else
        {
          self.ss = "the fred sheet you are currently using. filename = " + self.game.Data.RoadTypeObj[self.RoadNr].SheetFileName;
          let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.Data.RoadTypeObj[self.RoadNr].SheetSpriteID, tDescript: self.ss);
          self.txt1 = self.AddSubPart( tsubpart4, 10, 400, BitmapStore.GetWidth(self.game.Data.RoadTypeObj[self.RoadNr].SheetSpriteID), BitmapStore.Getheight(self.game.Data.RoadTypeObj[self.RoadNr].SheetSpriteID), 1);
        }
      }
    }

     void maketabsheetnr0b()
    {
      if (self.BBasicSpriteId > 0)
        self.RemoveSubPart(self.BBasicSpriteId);
      if (self.BChangeBasicSpriteId > 0)
        self.RemoveSubPart(self.BChangeBasicSpriteId);
      if (self.B5Id > 0)
        self.RemoveSubPart(self.B5Id);
      if (self.B5TextId > 0)
        self.RemoveSubPart(self.B5TextId);
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      if (self.B7Id > 0)
        self.RemoveSubPart(self.B7Id);
      if (self.B7TextId > 0)
        self.RemoveSubPart(self.B7TextId);
      self.ss = "Click to change the sprite to another graphic";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[self.DetailNr], tDescript: self.ss);
      self.BBasicSpriteId = self.AddSubPart( tsubpart1, 400, 350, 64, 48, 0);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BChangeBasicSpriteId = self.AddSubPart( tsubpart2, 400, 410, 32, 16, 1);
      }
      if (self.game.Data.Product < 7)
        return;
      self.ss = "";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.B4Id = self.AddSubPart( tsubpart3, 410, 610, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("Center6use=" + Conversion.Str( self.game.Data.RoadTypeObj[self.RoadNr].useCenter6), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B4TextId = self.AddSubPart( tsubpart4, 450, 609, 400, 20, 0);
      tsubpart4 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.B5Id = self.AddSubPart( tsubpart4, 410, 630, 32, 16, 1);
      tsubpart4 =  TextPartClass::new("Center6sprite=" + self.game.Data.RoadTypeObj[self.RoadNr].center6spriteFileName, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B5TextId = self.AddSubPart( tsubpart4, 450, 629, 400, 20, 0);
      tsubpart4 =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.B7Id = self.AddSubPart( tsubpart4, 410, 650, 32, 16, 1);
      tsubpart4 =  TextPartClass::new("Render to 64 sheet", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B7TextId = self.AddSubPart( tsubpart4, 450, 649, 400, 20, 0);
    }

     void maketabsheetnr1()
    {
      if (self.BasicList2Id > 0)
        self.RemoveSubPart(self.BasicList2Id);
      if (self.ChangeMvId > 0)
        self.RemoveSubPart(self.ChangeMvId);
      self.BasicList2Obj = ListClass::new();
      let mut index: i32 = 0;
      do
      {
        self.BasicList2Obj.add(self.game.Data.TempString[index] + "(" + Conversion.Str( index) + ") = " + Conversion.Str( self.game.Data.RoadTypeObj[self.RoadNr].MoveCostOverrule[index]) + "ap", index);
        index += 1;
      }
      while (index <= 99);
      ListClass basicList2Obj = self.BasicList2Obj;
      let mut detailNr: i32 = self.DetailNr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(basicList2Obj, 10, 300, detailNr, game, tHeader: "MoveCost for MoveTypes", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
      self.BasicList2Id = self.AddSubPart( tsubpart, 10, 350, 300, 208, 0);
      if (self.DetailNr > 99)
        self.DetailNr = -1;
      if (self.DetailNr <= -1)
        return;
      self.maketabsheetnr1b();
    }

     void maketabsheetnr1b()
    {
      self.ss = "Click to change move cost for selected movetype in AP";
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.ChangeMvId = self.AddSubPart( tsubpart, 400, 410, 32, 16, 1);
    }

     void maketabsheetnr2()
    {
      self.ss = "Click to set the thickness in pixel of the line used to draw road on str.map.";
      SubPartClass tsubpart;
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.bthick = self.AddSubPart( tsubpart, 10, 370, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Thickness=" + Conversion.Str( self.game.Data.RoadTypeObj[self.RoadNr].Thickness), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.bthicktext = self.AddSubPart( tsubpart, 50, 369, 400, 20, 0);
      self.ss = "Click to set that first this roadtype is drawn as another roadtype. -1= do not use.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.bother = self.AddSubPart( tsubpart, 10, 390, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("FirstOtherRoad=" + Conversion.Str( self.game.Data.RoadTypeObj[self.RoadNr].FirstDrawOther), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.bothertext = self.AddSubPart( tsubpart, 50, 389, 400, 20, 0);
      self.ss = "-1= do not use. Roadtypes that have the same category will flow into eachother with hex rendering.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B3Id = self.AddSubPart( tsubpart, 10, 410, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Category=" + Conversion.Str( self.game.Data.RoadTypeObj[self.RoadNr].Category), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B3TextId = self.AddSubPart( tsubpart, 50, 409, 400, 20, 0);
      self.ss = "0= do not use. ";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B6Id = self.AddSubPart( tsubpart, 10, 430, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Traffic Points=" + Conversion.Str( self.game.Data.RoadTypeObj[self.RoadNr].trafficPoints), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B6TextId = self.AddSubPart( tsubpart, 50, 429, 400, 20, 0);
      self.ss = "if set to TRUE the sprites defined below will be used to render the bridge, instead of those specified in the bridge tab.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BridgeId = self.AddSubPart( tsubpart, 10, 470, 32, 16, 1);
      }
      tsubpart =  TextPartClass::new("Overrule Bridge Gfx =" + Conversion.Str( self.game.Data.RoadTypeObj[self.RoadNr].BridgeOverrule), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.BridgeTextId = self.AddSubPart( tsubpart, 50, 469, 400, 20, 0);
      self.BridgeListObj = ListClass::new();
      let mut tdata: i32 = 0;
      do
      {
        self.BridgeListObj.add(self.game.Data.RoadTypeObj[self.RoadNr].BridgeOverruleSpriteFileName[tdata], tdata);
        tdata += 1;
      }
      while (tdata <= 5);
      ListClass bridgeListObj = self.BridgeListObj;
      let mut bridgeNr: i32 = self.BridgeNr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      tsubpart =  new ListSubPartClass(bridgeListObj, 7, 400, bridgeNr, game, tHeader: "Sprites", tbackbitmap: ( local1), bbx: 10, bby: 550, overruleFont: ( local2));
      self.BridgeListId = self.AddSubPart( tsubpart, 10, 550, 400, 160, 0);
      if (self.BridgeNr > 5)
        self.BridgeNr = -1;
      if (self.BridgeNr <= -1)
        return;
      self.maketabsheetnr2b();
    }

     void maketabsheetnr2b()
    {
      if (self.BBridgeSpriteId > 0)
        self.RemoveSubPart(self.BBridgeSpriteId);
      if (self.BChangeBridgeSpriteId > 0)
        self.RemoveSubPart(self.BChangeBridgeSpriteId);
      if (self.BChangeBridgeSpriteId2 > 0)
        self.RemoveSubPart(self.BChangeBridgeSpriteId2);
      self.ss = "Click to change the sprite to another graphic";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.Data.RoadTypeObj[self.RoadNr].BridgeOverruleSpriteID[self.BridgeNr], tDescript: self.ss);
      self.BBridgeSpriteId = self.AddSubPart( tsubpart1, 500, 450, 64, 48, 0);
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BChangeBridgeSpriteId = self.AddSubPart( tsubpart2, 500, 510, 32, 16, 1);
      }
      self.ss = "Import settings from another roadType";
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
      self.BChangeBridgeSpriteId2 = self.AddSubPart( tsubpart3, 500, 610, 32, 16, 1);
    }

    pub fn ConstructTileset(s: String)
    {
      strArray: Vec<String> = new string[65];
      bitmap1: Bitmap = new Bitmap(384, 528);
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics1 = Graphics.FromImage((Image) bitmap1);
      graphics1.Clear(Color.Transparent);
      graphics1.CompositingMode = CompositingMode.SourceOver;
      let mut num1: i32 = 64;
      let mut num2: i32 = 48;
      let mut index1: i32 = 0;
      num3: i32;
      do
      {
        let mut index2: i32 = 0;
        do
        {
          let mut index3: i32 = 0;
          do
          {
            let mut index4: i32 = 0;
            do
            {
              let mut index5: i32 = 0;
              do
              {
                let mut index6: i32 = 0;
                do
                {
                  let mut num4: i32 = 0;
                  let mut index7: i32 = self.game.SPRITE64[index1, index2, index3, index4, index5, index6];
                  if (index1 > 0 | index2 > 0 | index3 > 0 | index4 > 0 | index5 > 0 | index6 > 0)
                    graphics1.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].center6spriteId), self.game.SHEETX[index7] * num1, self.game.SHEETY[index7] * num2);
                  if (index1 == 1)
                  {
                    graphics1.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[0]), self.game.SHEETX[index7] * num1, self.game.SHEETY[index7] * num2);
                    num4 += 1;
                  }
                  if (index2 == 1)
                  {
                    graphics1.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[1]), self.game.SHEETX[index7] * num1, self.game.SHEETY[index7] * num2);
                    num4 += 1;
                  }
                  if (index3 == 1)
                  {
                    graphics1.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[2]), self.game.SHEETX[index7] * num1, self.game.SHEETY[index7] * num2);
                    num4 += 1;
                  }
                  if (index4 == 1)
                  {
                    graphics1.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[3]), self.game.SHEETX[index7] * num1, self.game.SHEETY[index7] * num2);
                    num4 += 1;
                  }
                  if (index5 == 1)
                  {
                    graphics1.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[4]), self.game.SHEETX[index7] * num1, self.game.SHEETY[index7] * num2);
                    num4 += 1;
                  }
                  if (index6 == 1)
                  {
                    graphics1.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[5]), self.game.SHEETX[index7] * num1, self.game.SHEETY[index7] * num2);
                    num3 = num4 + 1;
                  }
                  index6 += 1;
                }
                while (index6 <= 1);
                index5 += 1;
              }
              while (index5 <= 1);
              index4 += 1;
            }
            while (index4 <= 1);
            index3 += 1;
          }
          while (index3 <= 1);
          index2 += 1;
        }
        while (index2 <= 1);
        index1 += 1;
      }
      while (index1 <= 1);
      FileStream fileStream1 = new FileStream(s, FileMode.Create);
      bitmap1.Save((Stream) fileStream1, ImageFormat.Png);
      fileStream1.Close();
      graphics1.Dispose();
      bitmap2: Bitmap = new Bitmap(768, 1056);
      bitmap2.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics2 = Graphics.FromImage((Image) bitmap2);
      graphics2.Clear(Color.Transparent);
      graphics2.CompositingMode = CompositingMode.SourceOver;
      let mut num5: i32 = 128;
      let mut num6: i32 = 96;
      let mut index8: i32 = 0;
      do
      {
        let mut index9: i32 = 0;
        do
        {
          let mut index10: i32 = 0;
          do
          {
            let mut index11: i32 = 0;
            do
            {
              let mut index12: i32 = 0;
              do
              {
                let mut index13: i32 = 0;
                do
                {
                  let mut num7: i32 = 0;
                  let mut index14: i32 = self.game.SPRITE64[index8, index9, index10, index11, index12, index13];
                  if (index8 > 0 | index9 > 0 | index10 > 0 | index11 > 0 | index12 > 0 | index13 > 0)
                    graphics2.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].center6spriteId, 1), self.game.SHEETX[index14] * num5, self.game.SHEETY[index14] * num6);
                  if (index8 == 1)
                  {
                    graphics2.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[0], 1), self.game.SHEETX[index14] * num5, self.game.SHEETY[index14] * num6);
                    num7 += 1;
                  }
                  if (index9 == 1)
                  {
                    graphics2.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[1], 1), self.game.SHEETX[index14] * num5, self.game.SHEETY[index14] * num6);
                    num7 += 1;
                  }
                  if (index10 == 1)
                  {
                    graphics2.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[2], 1), self.game.SHEETX[index14] * num5, self.game.SHEETY[index14] * num6);
                    num7 += 1;
                  }
                  if (index11 == 1)
                  {
                    graphics2.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[3], 1), self.game.SHEETX[index14] * num5, self.game.SHEETY[index14] * num6);
                    num7 += 1;
                  }
                  if (index12 == 1)
                  {
                    graphics2.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[4], 1), self.game.SHEETX[index14] * num5, self.game.SHEETY[index14] * num6);
                    num7 += 1;
                  }
                  if (index13 == 1)
                  {
                    graphics2.DrawImage((Image) BitmapStore.GetBitmap(self.game.Data.RoadTypeObj[self.RoadNr].BasicSpriteID[5], 1), self.game.SHEETX[index14] * num5, self.game.SHEETY[index14] * num6);
                    num3 = num7 + 1;
                  }
                  index13 += 1;
                }
                while (index13 <= 1);
                index12 += 1;
              }
              while (index12 <= 1);
              index11 += 1;
            }
            while (index11 <= 1);
            index10 += 1;
          }
          while (index10 <= 1);
          index9 += 1;
        }
        while (index9 <= 1);
        index8 += 1;
      }
      while (index8 <= 1);
      FileStream fileStream2 = new FileStream(s.Replace(".png", "_big.png"), FileMode.Create);
      bitmap2.Save((Stream) fileStream2, ImageFormat.Png);
      fileStream2.Close();
      graphics2.Dispose();
      bitmap2.Dispose();
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
            if (num1 == self.RoadListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.RoadNr = num2;
                self.MakeRoadTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BAddRoadId)
            {
              self.game.Data.AddRoadType();
              self.MakeRoadListGUI(self.RoadNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BNameId)
            {
              self.game.Data.RoadTypeObj[self.RoadNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              self.MakeRoadListGUI(self.RoadNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsListId)
            {
              let mut num3: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                self.TabSheetNr = num3;
                self.MakeRoadTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BRemoveRoadId)
            {
              self.game.Data.RemoveRoadType(self.RoadNr);
              self.MakeRoadListGUI(-1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BasicListId)
            {
              let mut num4: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num4 > -1)
              {
                self.DetailNr = num4;
                self.maketabsheetnr0b();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BridgeListId)
            {
              let mut num5: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num5 > -1)
              {
                self.BridgeNr = num5;
                self.maketabsheetnr2b();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BasicList2Id)
            {
              let mut num6: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num6 > -1)
              {
                self.DetailNr = num6;
                self.maketabsheetnr1b();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeMvId)
            {
              let mut num7: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new ap overrule please", "Shadow Empire : Planetary Conquest")));
              if (num7 > 0 & num7 <= 9999)
              {
                self.game.Data.RoadTypeObj[self.RoadNr].MoveCostOverrule[self.DetailNr] = num7;
              }
              else
              {
                let mut num8: i32 =  Interaction.MsgBox( "Value between 1 and 10000 please...", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BChangeBridgeSpriteId2)
            {
              str: String = Interaction.InputBox("Give roadtype slot # to copy ", "Shadow Empire : Planetary Conquest");
              let mut index2: i32 =  Math.Round(Conversion.Val(str));
              if (!Information.IsNothing( str) && str.Length > 0 & index2 >= 0 & index2 <= self.game.Data.RoadTypeCounter)
              {
                self.game.Data.RoadTypeObj[self.RoadNr].BridgeOverrule = self.game.Data.RoadTypeObj[index2].BridgeOverrule;
                let mut nr: i32 = 0;
                do
                {
                  if (Operators.CompareString(self.game.Data.RoadTypeObj[index2].BridgeOverruleSpriteFileName[nr], self.game.Data.RoadTypeObj[self.RoadNr].BridgeOverruleSpriteFileName[nr], false) != 0)
                    self.game.Data.RoadTypeObj[self.RoadNr].ReplaceBridgeOverruleSprite(nr, self.game.Data.RoadTypeObj[index2].BridgeOverruleSpriteFileName[nr]);
                  nr += 1;
                }
                while (nr <= 5);
              }
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BChangeBasicSpriteId)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png (*.Png)|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For Road Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.RoadTypeObj[self.RoadNr].ReplaceBasicSprite(self.DetailNr, filename);
              }
              else
              {
                let mut num9: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B5Id)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png (*.Png)|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For center6 sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.RoadTypeObj[self.RoadNr].ReplaceCenter6(filename);
              }
              else
              {
                let mut num10: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B7Id)
            {
              self.ConstructTileset(self.game.HandyFunctionsObj.SaveSomething("Png|*.png", "Give savename for new tileset.", "", false));
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BChangeBridgeSpriteId)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select File For Road Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.RoadTypeObj[self.RoadNr].ReplaceBridgeOverruleSprite(self.BridgeNr, filename);
              }
              else
              {
                let mut num11: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BDrawId)
            {
              self.game.EditObj.PencilType = 2;
              self.game.EditObj.PencilData1 = self.RoadNr;
              windowReturnClass.AddCommand(1, 13);
              windowReturnClass.AddCommand(2, 13);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B2Id)
            {
              self.game.Data.RoadTypeObj[self.RoadNr].Transparent = !self.game.Data.RoadTypeObj[self.RoadNr].Transparent;
              self.MakeRoadTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BridgeId)
            {
              self.game.Data.RoadTypeObj[self.RoadNr].BridgeOverrule = !self.game.Data.RoadTypeObj[self.RoadNr].BridgeOverrule;
              self.MakeRoadTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B4Id)
            {
              self.game.Data.RoadTypeObj[self.RoadNr].useCenter6 = !self.game.Data.RoadTypeObj[self.RoadNr].useCenter6;
              self.MakeRoadTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BEP)
            {
              let mut num12: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("TYPE: Give new EP cost", "Shadow Empire : Planetary Conquest")));
              if (num12 > -1 & num12 < 999999)
                self.game.Data.RoadTypeObj[self.RoadNr].EPCost = num12;
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B6Id)
            {
              let mut num13: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new Traffic Points", "Shadow Empire : Planetary Conquest")));
              if (num13 > -1 & num13 < 999999)
                self.game.Data.RoadTypeObj[self.RoadNr].trafficPoints = num13;
              self.MakeRoadTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B3Id)
            {
              let mut num14: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give road category# (-1=none)", "Shadow Empire : Planetary Conquest")));
              if (num14 >= -1 & num14 < 999999)
                self.game.Data.RoadTypeObj[self.RoadNr].Category = num14;
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.bthick)
            {
              let mut num15: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new pixel thickness", "Shadow Empire : Planetary Conquest")));
              if (num15 > 0 & num15 < 10)
                self.game.Data.RoadTypeObj[self.RoadNr].Thickness = num15;
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.bother)
            {
              let mut num16: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give first draw other road nr#. -1=dont use", "Shadow Empire : Planetary Conquest")));
              if (num16 >= -1 & num16 <= self.game.Data.RoadTypeCounter)
                self.game.Data.RoadTypeObj[self.RoadNr].FirstDrawOther = num16;
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b1id)
            {
              self.game.Data.RoadTypeObj[self.RoadNr].SpecialLayer = !self.game.Data.RoadTypeObj[self.RoadNr].SpecialLayer;
              if (!self.game.Data.RoadTypeObj[self.RoadNr].SpecialLayer)
              {
                self.game.Data.RoadTypeObj[self.RoadNr].UseSheet = false;
                self.game.Data.RoadTypeObj[self.RoadNr].SheetFileName = "systemgraphics/trans.bmp";
              }
              if (self.game.Data.RoadTypeObj[self.RoadNr].SpecialLayer)
              {
                if (Interaction.MsgBox( "Use Fred SpriteSheet?", MsgBoxStyle.YesNo) == MsgBoxResult.No)
                  self.game.Data.RoadTypeObj[self.RoadNr].UseSheet = false;
                else
                  self.game.Data.RoadTypeObj[self.RoadNr].UseSheet = true;
                if (!self.game.Data.RoadTypeObj[self.RoadNr].UseSheet)
                {
                  extstring: String = Interaction.InputBox("Give a graphical extension: .jpg, .png, .bmp");
                  dirstring: String = Interaction.InputBox("Give a directory name under the graphics directory", "Shadow Empire : Planetary Conquest");
                  if (File.Exists(self.game.AppPath + "graphics/" + dirstring + "/a1" + extstring))
                  {
                    self.game.Data.RoadTypeObj[self.RoadNr].AutoLoadSpecial(dirstring, extstring);
                    self.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  self.game.Data.RoadTypeObj[self.RoadNr].SpecialLayer = false;
                  let mut num17: i32 =  Interaction.MsgBox( "Could not find this dir... give it like 'sea' or 'africa/desert', make sure a1 is present.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select normal sized sheet (big & small will be auto-linked)", self.game.AppPath + "graphics\\", true);
                  if (File.Exists(self.game.AppPath + "graphics/" + filename))
                  {
                    self.game.Data.RoadTypeObj[self.RoadNr].ReplaceSpriteSheet(filename);
                    self.maketabsheet();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  let mut num18: i32 =  Interaction.MsgBox( "Could not find this file... ", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
              self.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
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
