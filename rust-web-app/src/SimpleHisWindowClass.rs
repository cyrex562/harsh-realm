// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleHisWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleHisWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     cellinfoid: i32;
     tableId: i32;
     loadId: i32;
     text1id: i32;
     detailnr: i32;
     noneId: i32;
     noneIdB: i32;
     versionid: i32;
     addId: i32;
     addReinf: i32;
     removeReinf: i32;
     RenameReinf: i32;
     RenameReinfb: i32;
     RemoveReinfb: i32;
     a1id: i32;
     a2id: i32;
     a3id: i32;
     removeId: i32;
     changeId: i32;
     exitId: i32;
     saveId: i32;
     editId: i32;
     removeIdb: i32;
     saveIdb: i32;
     tab1id: i32;
     tab2id: i32;
     tab3id: i32;
     tab4id: i32;
     exportCsv: i32;
     importCsv: i32;
     tab1idb: i32;
     tab2idb: i32;
     tab3idb: i32;
     tab4idb: i32;
     editIdb: i32;
     strId: i32;
     detailx: i32;
     detaily: i32;
     tabId: i32;
     StringListClass stringy;
     VarsStartOn: i32;
     bool AddNew;
     bool Change;
     currentPplNr: i32;
     currentUnitNr: i32;
     currentHisNr: i32;
     currentInstNr: i32;
     int[] ColIsSFTypeVar;
     oldTopX: i32;
     oldTopY: i32;
     masterfileStart: String;

    pub SimpleHisWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight, 9, tDoBorders: 1, tHeaderString: "Intermediate Historical Unit Editor")
    {
      self.ColIsSFTypeVar = new int[100];
      self.detailx = -1;
      self.detaily = -1;
      self.detailnr = -1;
      self.tabId = 1;
      self.currentPplNr = -1;
      self.currentUnitNr = -1;
      self.currentHisNr = -1;
      self.currentInstNr = -1;
      self.AddNew = false;
      self.Change = false;
      self.game.EditObj.TempSFType = -1;
      self.masterfileStart = self.game.Data.MasterFile;
      self.game.Data.MasterFile = "";
      self.DoStuff();
    }

    pub fn RefreshCellInfo()
    {
      txt: String;
      if (self.detaily == -1 | self.detailx == -1)
      {
        txt = "No cell selected";
      }
      else
      {
        txt = "(row" + self.detailx.ToString() + ",col" + self.detaily.ToString() + ")             " + self.stringy.ColumnName[self.detaily].ToUpper() + "                ";
        if (self.stringy.Data[self.detailx, self.detaily].Length > 0)
          txt += self.stringy.Data[self.detailx, self.detaily];
        else if (self.stringy.TempDesc[self.detailx, self.detaily].Length > 0)
          txt += self.stringy.TempDesc[self.detailx, self.detaily];
      }
      if (self.cellinfoid > 0)
      {
        self.RemoveSubPart(self.cellinfoid);
        self.cellinfoid = 0;
      }
      let mut tsubpart: SubPartClass =  TextPartClass::new(txt, self.game.MarcFont4, self.game.ScreenWidth - 323, 20, false, tMarc: true);
      self.cellinfoid = self.AddSubPart( tsubpart, 312, 152, self.game.ScreenWidth - 323, 20, 0);
    }

    pub fn DoRefresh()
    {
      if (!self.Change)
        return;
      if (self.tableId > 0)
        self.RemoveSubPart(self.tableId);
      self.tableId = 0;
      self.Change = false;
      self.currentUnitNr = -1;
      self.currentHisNr = -1;
      self.currentPplNr = -1;
      self.currentInstNr = -1;
      self.DoStuff();
    }

    pub fn PopUpRefresh()
    {
    }

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight, -1);
      if (self.versionid > 0)
        self.RemoveSubPart(self.versionid);
      if (self.addId > 0)
        self.RemoveSubPart(self.addId);
      if (self.removeId > 0)
        self.RemoveSubPart(self.removeId);
      if (self.editId > 0)
        self.RemoveSubPart(self.editId);
      if (self.saveId > 0)
        self.RemoveSubPart(self.saveId);
      if (self.removeIdb > 0)
        self.RemoveSubPart(self.removeIdb);
      if (self.editIdb > 0)
        self.RemoveSubPart(self.editIdb);
      if (self.saveIdb > 0)
        self.RemoveSubPart(self.saveIdb);
      if (self.exitId > 0)
        self.RemoveSubPart(self.exitId);
      if (self.a1id > 0)
        self.RemoveSubPart(self.a1id);
      if (self.a2id > 0)
        self.RemoveSubPart(self.a2id);
      if (self.a3id > 0)
        self.RemoveSubPart(self.a3id);
      if (self.loadId > 0)
        self.RemoveSubPart(self.loadId);
      if (self.tab1id > 0)
        self.RemoveSubPart(self.tab1id);
      if (self.tab2id > 0)
        self.RemoveSubPart(self.tab2id);
      if (self.tab3id > 0)
        self.RemoveSubPart(self.tab3id);
      if (self.tab4id > 0)
        self.RemoveSubPart(self.tab4id);
      if (self.tab1idb > 0)
        self.RemoveSubPart(self.tab1idb);
      if (self.tab2idb > 0)
        self.RemoveSubPart(self.tab2idb);
      if (self.tab3idb > 0)
        self.RemoveSubPart(self.tab3idb);
      if (self.tab4idb > 0)
        self.RemoveSubPart(self.tab4idb);
      if (self.noneId > 0)
        self.RemoveSubPart(self.noneId);
      if (self.noneIdB > 0)
        self.RemoveSubPart(self.noneIdB);
      if (self.addReinf > 0)
        self.RemoveSubPart(self.addReinf);
      if (self.removeReinf > 0)
        self.RemoveSubPart(self.removeReinf);
      if (self.RenameReinf > 0)
        self.RemoveSubPart(self.RenameReinf);
      if (self.RemoveReinfb > 0)
        self.RemoveSubPart(self.RemoveReinfb);
      if (self.RenameReinfb > 0)
        self.RemoveSubPart(self.RenameReinfb);
      if (self.cellinfoid > 0)
      {
        self.RemoveSubPart(self.cellinfoid);
        self.cellinfoid = 0;
      }
      if (self.exportCsv > 0)
        self.RemoveSubPart(self.exportCsv);
      if (self.importCsv > 0)
        self.RemoveSubPart(self.importCsv);
      if (self.listId > 0)
        self.RemoveSubPart(self.listId);
      if (self.game.Data.LibraryCounter == -1)
      {
        self.game.Data.AddLibrary();
        self.game.Data.LibraryObj[0].name = "New Historical Library";
        self.game.Data.LibraryObj[0].information = "no info specified";
        self.game.Data.LibraryObj[0].creator = "no creator specified";
      }
      if (self.game.Data.RegimeCounter == -1)
      {
        self.game.Data.AddRegime();
        self.game.Data.RegimeObj[0].Name = "Only regime in library";
      }
      self.game.Data.RegimeObj[0].libId.libSlot = 0;
      self.game.Data.RegimeObj[0].libId.id = -1;
      bool[] flagArray = new bool[self.game.Data.SmallPicCounter + 1];
      let mut smallPicCounter1: i32 = self.game.Data.SmallPicCounter;
      for (let mut index: i32 = 0; index <= smallPicCounter1; index += 1)
      {
        if (self.game.Data.SmallLibId[index].libSlot >= 0)
          flagArray[index] = true;
        self.game.Data.SmallLibId[index].libSlot = -1;
        self.game.Data.SmallLibId[index].id = -1;
      }
      let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
      for (let mut index1: i32 = 0; index1 <= historicalUnitCounter; index1 += 1)
      {
        if (self.game.Data.HistoricalUnitObj[index1].ModelMaster > -1)
          self.game.Data.HistoricalUnitObj[index1].People = self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitObj[index1].ModelMaster].People;
        if (self.game.Data.HistoricalUnitObj[index1].Model)
        {
          if (self.game.Data.HistoricalUnitObj[index1].SmallGfx > -1)
          {
            self.game.Data.SmallLibId[self.game.Data.HistoricalUnitObj[index1].SmallGfx].libSlot = 0;
            self.game.Data.SmallLibId[self.game.Data.HistoricalUnitObj[index1].SmallGfx].id = -1;
          }
          let mut index2: i32 = 0;
          do
          {
            if (self.game.Data.HistoricalUnitObj[index1].SubParts[index2] > -1 && self.game.Data.HistoricalUnitObj[index1].DesignationSmall[index2] > -1)
            {
              self.game.Data.SmallLibId[self.game.Data.HistoricalUnitObj[index1].DesignationSmall[index2]].libSlot = 0;
              self.game.Data.SmallLibId[self.game.Data.HistoricalUnitObj[index1].DesignationSmall[index2]].id = -1;
            }
            index2 += 1;
          }
          while (index2 <= 5);
        }
      }
      for (let mut smallPicCounter2: i32 = self.game.Data.SmallPicCounter; smallPicCounter2 >= 0; smallPicCounter2 += -1)
      {
        if (flagArray[smallPicCounter2] & self.game.Data.SmallLibId[smallPicCounter2].libSlot == -1)
          self.game.Data.RemoveSmallPic(smallPicCounter2);
      }
      DrawMod.DrawTextColouredMarc( graphics, "Name:", self.game.MarcFont4, num1 + 10, 60, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, self.game.Data.LibraryObj[0].name, self.game.MarcFont3, num1 + 10, 75, Color.White);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Change Library Name", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a1id = self.AddSubPart( tsubpart1, num1 + 10, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "Author:", self.game.MarcFont4, num1 + 310, 60, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, self.game.Data.LibraryObj[0].creator, self.game.MarcFont3, num1 + 310, 75, Color.White);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Change Author", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 310), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a2id = self.AddSubPart( tsubpart2, num1 + 310, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "Information:", self.game.MarcFont4, num1 + 610, 60, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, Strings.Left(self.game.Data.LibraryObj[0].information, 15) + "...", self.game.MarcFont3, num1 + 610, 75, Color.White);
      tsubpart2 =  new TextButtonPartClass("Change information", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 610), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a3id = self.AddSubPart( tsubpart2, num1 + 610, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "Version:", self.game.MarcFont4, num1 + 910, 60, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, self.game.Data.LibraryObj[0].version.ToString(), self.game.MarcFont3, num1 + 910, 75, Color.White);
      tsubpart2 =  new TextButtonPartClass("Change version", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 910), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.versionid = self.AddSubPart( tsubpart2, num1 + 910, 100, 190, 35, 1);
      if (self.detailx > -1 & self.detaily > -1 & !Information.IsNothing( self.stringy))
        self.RefreshCellInfo();
      let mut y1: i32 = 60;
      let mut num3: i32 = 40;
      DrawMod.DrawTextColouredMarc( graphics, "Ops:", self.game.MarcFont3, 40, y1, Color.White);
      let mut num4: i32 = y1 + 40;
      if (self.game.Data.PeopleCounter > -1)
      {
        tsubpart2 =  new TextButtonPartClass("Save Historical Library", 190, "Save a TroopType Library",  self.OwnBitmap, num3, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveId = self.AddSubPart( tsubpart2, num3, num4, 190, 35, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Save Historical Library", 190, "Nothing to save",  self.OwnBitmap, num3, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveIdb = self.AddSubPart( tsubpart2, num3, num4, 190, 35, 1);
      }
      let mut num5: i32 = num4 + 50;
      tsubpart2 =  new TextButtonPartClass("Load Historical Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: num5, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadId = self.AddSubPart( tsubpart2, num3, num5, 190, 35, 1);
      let mut num6: i32 = num5 + 50;
      tsubpart2 =  new TextButtonPartClass("Exit", 190, tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: num6, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.exitId = self.AddSubPart( tsubpart2, num3, num6, 190, 35, 1);
      let mut y2: i32 = num6 + 50;
      DrawMod.DrawTextColouredMarc( graphics, "Tabsheets:", self.game.MarcFont3, 40, y2, Color.White);
      let mut num7: i32 = y2 + 40;
      let mut num8: i32 = 20;
      if (self.tabId != 1)
      {
        tsubpart2 =  new TextButtonPartClass("Peoples", 190, "Click to go to people tabsheet",  self.OwnBitmap, num8 + 20, num7, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.tab1id = self.AddSubPart( tsubpart2, num8 + 20, num7, 190, 35, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Peoples", 190, "People tabsheet already on screen",  self.OwnBitmap, num8 + 20, num7, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.tab1idb = self.AddSubPart( tsubpart2, num8 + 20, num7, 190, 35, 1);
      }
      let mut num9: i32 = num7 + 50;
      if (self.tabId != 2)
      {
        tsubpart2 =  new TextButtonPartClass("Unit Composition", 190, "Click to go to unit composition tabsheet",  self.OwnBitmap, num8 + 20, num9, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.tab2id = self.AddSubPart( tsubpart2, num8 + 20, num9, 190, 35, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Unit Composition", 190, "Unit composition tabsheet already on screen",  self.OwnBitmap, num8 + 20, num9, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.tab2idb = self.AddSubPart( tsubpart2, num8 + 20, num9, 190, 35, 1);
      }
      let mut num10: i32 = num9 + 50;
      if (self.tabId != 3)
      {
        tsubpart2 =  new TextButtonPartClass("Historical Models", 190, "Click to go to Historical Models tabsheet",  self.OwnBitmap, num8 + 20, num10, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.tab3id = self.AddSubPart( tsubpart2, num8 + 20, num10, 190, 35, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Historical Models", 190, "Historical Models tabsheet already on screen",  self.OwnBitmap, num8 + 20, num10, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.tab3idb = self.AddSubPart( tsubpart2, num8 + 20, num10, 190, 35, 1);
      }
      let mut num11: i32 = num10 + 50;
      if (self.tabId != 4)
      {
        tsubpart2 =  new TextButtonPartClass("Historical Units", 190, "Click to go to Historical Units tabsheet",  self.OwnBitmap, num8 + 20, num11, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.tab4id = self.AddSubPart( tsubpart2, num8 + 20, num11, 190, 35, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Historical Units", 190, "Historical Units tabsheet already on screen",  self.OwnBitmap, num8 + 20, num11, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.tab4idb = self.AddSubPart( tsubpart2, num8 + 20, num11, 190, 35, 1);
      }
      let mut y3: i32 = num11 + 50;
      DrawMod.DrawTextColouredMarc( graphics, "TroopType Libraries:", self.game.MarcFont3, 40, y3, Color.White);
      let mut y4: i32 = y3 + 40;
      self.listObj = ListClass::new();
      let mut num12: i32 = -1;
      let mut num13: i32 = -1;
      let mut libraryCounter: i32 = self.game.Data.LibraryCounter;
      for (let mut tdata: i32 = 0; tdata <= libraryCounter; tdata += 1)
      {
        bool flag = false;
        let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
        for (let mut index: i32 = 0; index <= sfTypeCounter; index += 1)
        {
          if (self.game.Data.SFTypeObj[index].LibId.libSlot == tdata)
            flag = true;
        }
        if (flag)
        {
          num12 += 1;
          if (self.detailnr == tdata)
            num13 = num12;
          self.listObj.add(self.game.Data.LibraryObj[tdata].name, tdata);
        }
      }
      ListClass listObj = self.listObj;
      let mut tlistselect: i32 = num13;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      let mut bby: i32 = y4;
      font: Font =  null;
       local2: Font =  font;
      tsubpart2 =  new ListSubPartClass(listObj, 6, 210, tlistselect, game, true, "TroopType Libraries", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 40, bby: bby, tMarcStyle: true, overruleFont: ( local2));
      self.listId = self.AddSubPart( tsubpart2, 40, y4, 210, 144, 0);
      let mut y5: i32 = y4 + 135;
      tsubpart2 =  new TextButtonPartClass("Add Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 620, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addReinf = self.AddSubPart( tsubpart2, 40, y5, 190, 35, 1);
      let mut y6: i32 = y5 + 50;
      if (self.detailnr > -1)
      {
        tsubpart2 =  new TextButtonPartClass("Remove Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 660, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeReinf = self.AddSubPart( tsubpart2, 40, y6, 190, 35, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Remove Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: 10, bby: 660, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RemoveReinfb = self.AddSubPart( tsubpart2, 40, y6, 190, 35, 1);
      }
      if (self.tabId == 1)
        self.Tab1( graphics);
      if (self.tabId == 2)
        self.Tab2( graphics);
      if (self.tabId == 3)
        self.Tab3( graphics);
      if (self.tabId != 4)
        return;
      self.Tab4( graphics);
    }

    pub fn Tab2( Graphics g)
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      let mut num3: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - 322) / 24.0)) - 1;
      let mut num4: i32 = 172 + num3 * 24 + 56;
      if (self.tableId == 0)
      {
        self.stringy = new StringListClass(-1);
        let mut col1: i32 = 1 - 1;
        self.stringy.AddCol(col1, "Slot");
        let mut col2: i32 = col1 + 1;
        self.stringy.AddCol(col2, "ID#");
        let mut col3: i32 = col2 + 1;
        self.stringy.AddCol(col3, "Name");
        let mut col4: i32 = col3 + 1;
        self.stringy.AddCol(col4, "IsHQ");
        let mut index1: i32 = 1;
        do
        {
          let mut col5: i32 = col4 + 1;
          self.stringy.AddCol(col5, "Troop" + index1.ToString());
          let mut col6: i32 = col5 + 1;
          self.stringy.AddCol(col6, "People" + index1.ToString());
          let mut col7: i32 = col6 + 1;
          self.stringy.AddCol(col7, "Qty" + index1.ToString());
          let mut col8: i32 = col7 + 1;
          self.stringy.AddCol(col8, "Mor" + index1.ToString());
          col4 = col8 + 1;
          self.stringy.AddCol(col4, "Xp" + index1.ToString());
          index1 += 1;
        }
        while (index1 <= 8);
        let mut index2: i32 = -1;
        let mut unitCounter: i32 = self.game.Data.UnitCounter;
        for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
        {
          if (self.game.Data.UnitObj[index3].PreDef > -1)
          {
            index2 += 1;
            self.stringy.AddRow(index2 - 1);
            let mut index4: i32 = 1 - 1;
            self.stringy.Data[index2, index4] = index3.ToString();
            let mut index5: i32 = index4 + 1;
            self.stringy.Data[index2, index5] = self.game.Data.UnitObj[index3].PreDef.ToString();
            let mut index6: i32 = index5 + 1;
            self.stringy.Data[index2, index6] = self.game.Data.UnitObj[index3].Name;
            let mut index7: i32 = index6 + 1;
            self.stringy.Data[index2, index7] = self.game.Data.UnitObj[index3].IsHQ.ToString();
            index1 = 0;
            do
            {
              if (self.game.Data.UnitObj[index3].SFCount >= index1)
              {
                if (self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].Type == -1)
                {
                  self.game.Data.UnitObj[index3].RemoveSF(self.game.Data.UnitObj[index3].SFList[index1]);
                  --index1;
                }
                else if (self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].People == -1 && self.game.Data.PeopleCounter > -1)
                  self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].People = 0;
              }
              index1 += 1;
            }
            while (index1 <= 7);
            index1 = 0;
            do
            {
              if (self.game.Data.UnitObj[index3].SFCount >= index1)
              {
                let mut index8: i32 = index7 + 1;
                self.stringy.Data[index2, index8] = self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].Type <= -1 ? "none set" : self.game.Data.SFTypeObj[self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].Type].Name;
                let mut index9: i32 = index8 + 1;
                self.stringy.Data[index2, index9] = self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].People <= -1 ? "none set" : self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].People.ToString();
                let mut index10: i32 = index9 + 1;
                self.stringy.Data[index2, index10] = self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].Qty.ToString();
                let mut index11: i32 = index10 + 1;
                self.stringy.Data[index2, index11] = self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].Mor.ToString();
                index7 = index11 + 1;
                self.stringy.Data[index2, index7] = self.game.Data.SFObj[self.game.Data.UnitObj[index3].SFList[index1]].Xp.ToString();
              }
              else
              {
                let mut index12: i32 = index7 + 1;
                self.stringy.Data[index2, index12] = "";
                let mut index13: i32 = index12 + 1;
                self.stringy.Data[index2, index13] = "";
                let mut index14: i32 = index13 + 1;
                self.stringy.Data[index2, index14] = "";
                let mut index15: i32 = index14 + 1;
                self.stringy.Data[index2, index15] = "";
                index7 = index15 + 1;
                self.stringy.Data[index2, index7] = "";
              }
              index1 += 1;
            }
            while (index1 <= 7);
          }
        }
        let mut tsubpart: SubPartClass =  new MatrixSubPartClass(self.stringy, num3 - 1, self.game.ScreenWidth - 323, self.detailx, self.detaily, self.game, tbackbitmap: ( self.BackBitmap), bbx: 312, bby: 172, trowheight: 24, tfontsize: 16, tfontoffsety: 4, tMarcy: true);
        self.tableId = self.AddSubPart( tsubpart, 312, 172, self.game.ScreenWidth - 323, (num3 + 2) * 24, 0);
        if (self.oldTopX > 0 | self.oldTopY > 0)
        {
          ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemX = self.oldTopX;
          ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemY = self.oldTopY;
        }
      }
      else
      {
        self.oldTopX = ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemX;
        self.oldTopY = ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemY;
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Add Unit", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addId = self.AddSubPart( tsubpart1, num1 + 10, num4, 190, 35, 1);
      if (self.detailx > -1)
      {
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Remove Unit", 190, "Click and remove selected row",  self.OwnBitmap, num1 + 210, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeId = self.AddSubPart( tsubpart2, num1 + 210, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Remove Unit", 190, "No row selected",  self.OwnBitmap, num1 + 210, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeIdb = self.AddSubPart( tsubpart3, num1 + 210, num4, 190, 35, 1);
      }
      if (self.detailx > -1 & self.detaily > -1)
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "Click to edit selected value or text",  self.OwnBitmap, num1 + 410, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editId = self.AddSubPart( tsubpart4, num1 + 410, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "No cell selected",  self.OwnBitmap, num1 + 410, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editIdb = self.AddSubPart( tsubpart5, num1 + 410, num4, 190, 35, 1);
      }
      let mut unitCounter1: i32 = self.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter1; index += 1)
      {
        if (self.game.Data.UnitObj[index].PreDef > -1)
        {
          self.game.Data.UnitObj[index].LibId.libSlot = 0;
          self.game.Data.UnitObj[index].LibId.id = -1;
        }
      }
    }

    pub fn Tab3( Graphics g)
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      let mut num3: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - 322) / 24.0)) - 1;
      let mut num4: i32 = 172 + num3 * 24 + 56;
      if (self.tableId == 0)
      {
        self.stringy = new StringListClass(-1);
        let mut col1: i32 = 1 - 1;
        self.stringy.AddCol(col1, "Slot");
        let mut col2: i32 = col1 + 1;
        self.stringy.AddCol(col2, "ID#");
        let mut col3: i32 = col2 + 1;
        self.stringy.AddCol(col3, "Name");
        let mut col4: i32 = col3 + 1;
        self.stringy.AddCol(col4, "Level");
        let mut col5: i32 = col4 + 1;
        self.stringy.AddCol(col5, "Small Gfx");
        let mut col6: i32 = col5 + 1;
        self.stringy.AddCol(col6, "Shortname");
        let mut col7: i32 = col6 + 1;
        self.stringy.AddCol(col7, "Pol.Pts");
        let mut col8: i32 = col7 + 1;
        self.stringy.AddCol(col8, "GfxSymbolUsePeople");
        let mut col9: i32 = col8 + 1;
        self.stringy.AddCol(col9, "NameCounter");
        let mut col10: i32 = col9 + 1;
        self.stringy.AddCol(col10, "CounterRomans");
        let mut col11: i32 = col10 + 1;
        self.stringy.AddCol(col11, "UseModelCounter");
        let mut col12: i32 = col11 + 1;
        self.stringy.AddCol(col12, "People");
        let mut col13: i32 = col12 + 1;
        self.stringy.AddCol(col13, "Max Present");
        let mut col14: i32 = col13 + 1;
        self.stringy.AddCol(col14, "Battlegroup");
        let mut col15: i32 = col14 + 1;
        self.stringy.AddCol(col15, "Unit1");
        let mut col16: i32 = col15 + 1;
        self.stringy.AddCol(col16, "Gfx1");
        let mut col17: i32 = col16 + 1;
        self.stringy.AddCol(col17, "Unit2");
        let mut col18: i32 = col17 + 1;
        self.stringy.AddCol(col18, "Gfx2");
        let mut col19: i32 = col18 + 1;
        self.stringy.AddCol(col19, "Unit3");
        let mut col20: i32 = col19 + 1;
        self.stringy.AddCol(col20, "Gfx3");
        let mut col21: i32 = col20 + 1;
        self.stringy.AddCol(col21, "Unit4");
        let mut col22: i32 = col21 + 1;
        self.stringy.AddCol(col22, "Gfx4");
        let mut col23: i32 = col22 + 1;
        self.stringy.AddCol(col23, "Unit5");
        let mut col24: i32 = col23 + 1;
        self.stringy.AddCol(col24, "Gfx5");
        let mut col25: i32 = col24 + 1;
        self.stringy.AddCol(col25, "Unit6");
        self.stringy.AddCol(col25 + 1, "Gfx6");
        let mut index1: i32 = -1;
        let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
        for (let mut index2: i32 = 0; index2 <= historicalUnitCounter; index2 += 1)
        {
          if (self.game.Data.HistoricalUnitObj[index2].Model)
          {
            index1 += 1;
            self.stringy.AddRow(index1 - 1);
            let mut index3: i32 = 1 - 1;
            self.stringy.Data[index1, index3] = index2.ToString();
            let mut index4: i32 = index3 + 1;
            self.stringy.Data[index1, index4] = self.game.Data.HistoricalUnitObj[index2].ID.ToString();
            let mut index5: i32 = index4 + 1;
            self.stringy.Data[index1, index5] = self.game.Data.HistoricalUnitObj[index2].Name;
            let mut index6: i32 = index5 + 1;
            self.stringy.Data[index1, index6] = "";
            if (self.game.Data.HistoricalUnitObj[index2].Type == 1)
              self.stringy.Data[index1, index6] = "Ind Unit";
            if (self.game.Data.HistoricalUnitObj[index2].Type == 2)
              self.stringy.Data[index1, index6] = "Multi Unit";
            if (self.game.Data.HistoricalUnitObj[index2].Type == 5)
              self.stringy.Data[index1, index6] = "Lowest HQ";
            if (self.game.Data.HistoricalUnitObj[index2].Type == 6)
              self.stringy.Data[index1, index6] = "Medium HQ";
            if (self.game.Data.HistoricalUnitObj[index2].Type == 7)
              self.stringy.Data[index1, index6] = "High HQ";
            if (self.game.Data.HistoricalUnitObj[index2].Type == 8)
              self.stringy.Data[index1, index6] = "Supreme HQ";
            let mut index7: i32 = index6 + 1;
            if (self.game.Data.HistoricalUnitObj[index2].SmallGfx == -1)
              self.stringy.Data[index1, index7] = "none";
            if (self.game.Data.HistoricalUnitObj[index2].SmallGfx > -1)
            {
              self.stringy.TempBmp[index1, index7] = self.game.Data.SmallPicNr[self.game.Data.HistoricalUnitObj[index2].SmallGfx];
              self.stringy.TempDesc[index1, index7] = "SmallGfxNr" + self.game.Data.HistoricalUnitObj[index2].SmallGfx.ToString();
            }
            let mut index8: i32 = index7 + 1;
            self.stringy.Data[index1, index8] = self.game.Data.HistoricalUnitObj[index2].CounterString;
            let mut index9: i32 = index8 + 1;
            self.stringy.Data[index1, index9] = self.game.Data.HistoricalUnitObj[index2].PP.ToString();
            let mut index10: i32 = index9 + 1;
            self.stringy.Data[index1, index10] = self.game.Data.HistoricalUnitObj[index2].UsePeopleGfx <= -1 ? "none" : self.game.Data.PeopleObj[self.game.Data.HistoricalUnitObj[index2].UsePeopleGfx].Name;
            let mut index11: i32 = index10 + 1;
            self.stringy.Data[index1, index11] = self.game.Data.HistoricalUnitObj[index2].NameCounter.ToString();
            let mut index12: i32 = index11 + 1;
            self.stringy.Data[index1, index12] = self.game.Data.HistoricalUnitObj[index2].UseRomans.ToString();
            let mut index13: i32 = index12 + 1;
            self.stringy.Data[index1, index13] = self.game.Data.HistoricalUnitObj[index2].UseModelCounter <= -1 ? "none" : self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitObj[index2].UseModelCounter].Name;
            let mut index14: i32 = index13 + 1;
            self.stringy.Data[index1, index14] = self.game.Data.HistoricalUnitObj[index2].People <= -1 ? "none" : self.game.Data.PeopleObj[self.game.Data.HistoricalUnitObj[index2].People].Name;
            let mut index15: i32 = index14 + 1;
            self.stringy.Data[index1, index15] = self.game.Data.HistoricalUnitObj[index2].MaxPresent.ToString();
            let mut index16: i32 = index15 + 1;
            self.stringy.Data[index1, index16] = self.game.Data.HistoricalUnitObj[index2].BattleGroup.ToString();
            let mut index17: i32 = 0;
            do
            {
              if (self.game.Data.HistoricalUnitObj[index2].SubParts[index17] > -1)
              {
                let mut index18: i32 = index16 + 1;
                self.stringy.Data[index1, index18] = self.game.Data.HistoricalUnitObj[index2].SubParts[index17] <= -1 ? "none" : self.game.Data.UnitObj[self.game.HandyFunctionsObj.GetPreDef(self.game.Data.HistoricalUnitObj[index2].SubParts[index17])].Name;
                index16 = index18 + 1;
                if (self.game.Data.HistoricalUnitObj[index2].DesignationSmall[index17] == -1)
                  self.stringy.Data[index1, index16] = "none";
                if (self.game.Data.HistoricalUnitObj[index2].DesignationSmall[index17] > -1)
                {
                  self.stringy.Data[index1, index16] = "";
                  self.stringy.TempBmp[index1, index16] = self.game.Data.SmallPicNr[self.game.Data.HistoricalUnitObj[index2].DesignationSmall[index17]];
                }
              }
              else
              {
                let mut index19: i32 = index16 + 1;
                self.stringy.Data[index1, index19] = "";
                index16 = index19 + 1;
                self.stringy.Data[index1, index16] = "";
              }
              index17 += 1;
            }
            while (index17 <= 5);
          }
        }
        let mut tsubpart: SubPartClass =  new MatrixSubPartClass(self.stringy, num3 - 1, self.game.ScreenWidth - 323, self.detailx, self.detaily, self.game, tbackbitmap: ( self.BackBitmap), bbx: 312, bby: 172, trowheight: 24, tfontsize: 16, tfontoffsety: 4, tMarcy: true);
        self.tableId = self.AddSubPart( tsubpart, 312, 172, self.game.ScreenWidth - 323, num3 * 25, 0);
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Add Model", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addId = self.AddSubPart( tsubpart1, num1 + 10, num4, 190, 35, 1);
      if (self.detailx > -1)
      {
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Remove Model", 190, "Click and remove selected row",  self.OwnBitmap, num1 + 210, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeId = self.AddSubPart( tsubpart2, num1 + 210, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Remove Model", 190, "No row selected",  self.OwnBitmap, num1 + 210, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeIdb = self.AddSubPart( tsubpart3, num1 + 210, num4, 190, 35, 1);
      }
      if (self.detailx > -1 & self.detaily > -1)
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "Click to edit selected value or text",  self.OwnBitmap, num1 + 410, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editId = self.AddSubPart( tsubpart4, num1 + 410, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "No cell selected",  self.OwnBitmap, num1 + 410, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editIdb = self.AddSubPart( tsubpart5, num1 + 410, num4, 190, 35, 1);
      }
      if (self.detaily == 4 | self.detaily == 14 | self.detaily == 16 | self.detaily == 18 | self.detaily == 20 | self.detaily == 22 | self.detaily == 24)
      {
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Clear Cell", 190, "Click to remove Gfx in this cell",  self.OwnBitmap, num1 + 610, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.noneId = self.AddSubPart( tsubpart6, num1 + 610, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Clear Cell", 190, "No valid gfx cell selected",  self.OwnBitmap, num1 + 610, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.noneIdB = self.AddSubPart( tsubpart7, num1 + 610, num4, 190, 35, 1);
      }
      let mut historicalUnitCounter1: i32 = self.game.Data.HistoricalUnitCounter;
      for (let mut index: i32 = 0; index <= historicalUnitCounter1; index += 1)
      {
        self.game.Data.HistoricalUnitObj[index].LibId.libSlot = 0;
        self.game.Data.HistoricalUnitObj[index].LibId.id = -1;
      }
    }

    pub fn Tab4( Graphics g)
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      let mut num3: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - 322) / 24.0)) - 1;
      let mut num4: i32 = 172 + num3 * 24 + 56;
      if (self.tableId == 0)
      {
        self.stringy = new StringListClass(-1);
        let mut col1: i32 = 1 - 1;
        self.stringy.AddCol(col1, "Slot");
        let mut col2: i32 = col1 + 1;
        self.stringy.AddCol(col2, "ID#");
        let mut col3: i32 = col2 + 1;
        self.stringy.AddCol(col3, "Name");
        let mut col4: i32 = col3 + 1;
        self.stringy.AddCol(col4, "Based On Model");
        let mut col5: i32 = col4 + 1;
        self.stringy.AddCol(col5, "Shortname");
        let mut col6: i32 = col5 + 1;
        self.stringy.AddCol(col6, "tv1");
        let mut col7: i32 = col6 + 1;
        self.stringy.AddCol(col7, "tv2");
        let mut col8: i32 = col7 + 1;
        self.stringy.AddCol(col8, "tv3");
        let mut col9: i32 = col8 + 1;
        self.stringy.AddCol(col9, "tv4");
        self.stringy.AddCol(col9 + 1, "tv5");
        let mut index1: i32 = -1;
        let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
        for (let mut index2: i32 = 0; index2 <= historicalUnitCounter; index2 += 1)
        {
          if (!self.game.Data.HistoricalUnitObj[index2].Model)
          {
            index1 += 1;
            self.stringy.AddRow(index1 - 1);
            let mut index3: i32 = 1 - 1;
            self.stringy.Data[index1, index3] = index2.ToString();
            let mut index4: i32 = index3 + 1;
            self.stringy.Data[index1, index4] = self.game.Data.HistoricalUnitObj[index2].ID.ToString();
            let mut index5: i32 = index4 + 1;
            self.stringy.Data[index1, index5] = self.game.Data.HistoricalUnitObj[index2].Name;
            let mut index6: i32 = index5 + 1;
            self.stringy.Data[index1, index6] = self.game.Data.HistoricalUnitObj[index2].ModelMaster <= -1 ? "none" : self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitObj[index2].ModelMaster].Name;
            let mut index7: i32 = index6 + 1;
            self.stringy.Data[index1, index7] = self.game.Data.HistoricalUnitObj[index2].CounterString;
            let mut index8: i32 = index7 + 1;
            self.stringy.Data[index1, index8] = self.game.Data.HistoricalUnitObj[index2].TempVar1.ToString();
            let mut index9: i32 = index8 + 1;
            self.stringy.Data[index1, index9] = self.game.Data.HistoricalUnitObj[index2].TempVar2.ToString();
            let mut index10: i32 = index9 + 1;
            self.stringy.Data[index1, index10] = self.game.Data.HistoricalUnitObj[index2].TempVar3.ToString();
            let mut index11: i32 = index10 + 1;
            self.stringy.Data[index1, index11] = self.game.Data.HistoricalUnitObj[index2].TempVar4.ToString();
            let mut index12: i32 = index11 + 1;
            self.stringy.Data[index1, index12] = self.game.Data.HistoricalUnitObj[index2].TempVar5.ToString();
          }
        }
        let mut tsubpart: SubPartClass =  new MatrixSubPartClass(self.stringy, num3 - 1, self.game.ScreenWidth - 323, self.detailx, self.detaily, self.game, tbackbitmap: ( self.BackBitmap), bbx: 312, bby: 172, trowheight: 24, tfontsize: 16, tfontoffsety: 4, tMarcy: true);
        self.tableId = self.AddSubPart( tsubpart, 312, 172, self.game.ScreenWidth - 323, num3 * 23, 0);
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Add His Unit", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addId = self.AddSubPart( tsubpart1, num1 + 10, num4, 190, 35, 1);
      if (self.detailx > -1)
      {
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Remove His Unit", 190, "Click and remove selected row",  self.OwnBitmap, num1 + 210, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeId = self.AddSubPart( tsubpart2, num1 + 210, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Remove His Unit", 190, "No row selected",  self.OwnBitmap, num1 + 210, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeIdb = self.AddSubPart( tsubpart3, num1 + 210, num4, 190, 35, 1);
      }
      if (self.detailx > -1 & self.detaily > -1)
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "Click to edit selected value or text",  self.OwnBitmap, num1 + 410, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editId = self.AddSubPart( tsubpart4, num1 + 410, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "No cell selected",  self.OwnBitmap, num1 + 410, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editIdb = self.AddSubPart( tsubpart5, num1 + 410, num4, 190, 35, 1);
      }
      let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Export CSV", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 610), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.exportCsv = self.AddSubPart( tsubpart6, num1 + 610, num4, 190, 35, 1);
      let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Import CSV", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 810), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.importCsv = self.AddSubPart( tsubpart7, num1 + 810, num4, 190, 35, 1);
      let mut historicalUnitCounter1: i32 = self.game.Data.HistoricalUnitCounter;
      for (let mut index: i32 = 0; index <= historicalUnitCounter1; index += 1)
      {
        self.game.Data.HistoricalUnitObj[index].LibId.libSlot = 0;
        self.game.Data.HistoricalUnitObj[index].LibId.id = -1;
      }
    }

    pub fn Tab1( Graphics g)
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      let mut num3: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - 322) / 24.0)) - 1;
      let mut num4: i32 = 172 + num3 * 24 + 56;
      if (self.tableId == 0)
      {
        self.stringy = new StringListClass(-1);
        let mut col1: i32 = 1 - 1;
        self.stringy.AddCol(col1, "Slot# / Id#");
        let mut col2: i32 = col1 + 1;
        self.stringy.AddCol(col2, "Name");
        let mut col3: i32 = col2 + 1;
        self.stringy.AddCol(col3, "BreakAt");
        let mut col4: i32 = col3 + 1;
        self.stringy.AddCol(col4, "People Sideways Gfx");
        let mut col5: i32 = col4 + 1;
        self.stringy.AddCol(col5, "People Symbol Gfx");
        let mut col6: i32 = col5 + 1;
        self.stringy.AddCol(col6, "Base Morale");
        let mut col7: i32 = col6 + 1;
        self.stringy.AddCol(col7, "Color");
        self.stringy.AddCol(col7 + 1, "ArtCode");
        let mut index1: i32 = -1;
        let mut peopleCounter: i32 = self.game.Data.PeopleCounter;
        for (let mut index2: i32 = 0; index2 <= peopleCounter; index2 += 1)
        {
          index1 += 1;
          self.stringy.AddRow(index1 - 1);
          let mut index3: i32 = 1 - 1;
          self.stringy.Data[index1, index3] = index2.ToString() + "/" + self.game.Data.PeopleObj[index2].id.ToString();
          let mut index4: i32 = index3 + 1;
          self.stringy.Data[index1, index4] = self.game.Data.PeopleObj[index2].Name;
          let mut index5: i32 = index4 + 1;
          self.stringy.Data[index1, index5] = self.game.Data.PeopleObj[index2].BreakAt.ToString();
          let mut index6: i32 = index5 + 1;
          self.stringy.Data[index1, index6] = self.game.Data.PeopleObj[index2].SidewaysFileName;
          let mut index7: i32 = index6 + 1;
          self.stringy.Data[index1, index7] = self.game.Data.PeopleObj[index2].NationalFileName;
          let mut index8: i32 = index7 + 1;
          self.stringy.Data[index1, index8] = self.game.Data.PeopleObj[index2].BaseMorale[0].ToString();
          let mut index9: i32 = index8 + 1;
          if (self.game.Data.PeopleObj[index2].Red > -1)
            self.stringy.Data[index1, index9] = self.game.Data.PeopleObj[index2].Red.ToString() + "," + self.game.Data.PeopleObj[index2].Green.ToString() + "," + self.game.Data.PeopleObj[index2].Blue.ToString();
          else
            self.stringy.Data[index1, index9] = "No color";
          let mut index10: i32 = index9 + 1;
          self.stringy.Data[index1, index10] = self.game.Data.PeopleObj[index2].artCode.ToString();
        }
        let mut tsubpart: SubPartClass =  new MatrixSubPartClass(self.stringy, num3 - 1, self.game.ScreenWidth - 323, self.detailx, self.detaily, self.game, tbackbitmap: ( self.BackBitmap), bbx: 312, bby: 172, trowheight: 24, tfontsize: 16, tfontoffsety: 4, tMarcy: true);
        self.tableId = self.AddSubPart( tsubpart, 312, 172, self.game.ScreenWidth - 323, num3 * 23, 0);
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Add People", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addId = self.AddSubPart( tsubpart1, num1 + 10, num4, 190, 35, 1);
      if (self.detailx > -1)
      {
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Remove People", 190, "Click and remove selected row",  self.OwnBitmap, num1 + 210, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeId = self.AddSubPart( tsubpart2, num1 + 210, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Remove People", 190, "No row selected",  self.OwnBitmap, num1 + 210, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeIdb = self.AddSubPart( tsubpart3, num1 + 210, num4, 190, 35, 1);
      }
      if (self.detailx > -1 & self.detaily > -1)
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "Click to edit selected value or text",  self.OwnBitmap, num1 + 410, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editId = self.AddSubPart( tsubpart4, num1 + 410, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "No cell selected",  self.OwnBitmap, num1 + 410, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editIdb = self.AddSubPart( tsubpart5, num1 + 410, num4, 190, 35, 1);
      }
      let mut peopleCounter1: i32 = self.game.Data.PeopleCounter;
      for (let mut index: i32 = 0; index <= peopleCounter1; index += 1)
      {
        self.game.Data.PeopleObj[index].LibId.libSlot = 0;
        self.game.Data.PeopleObj[index].LibId.id = -1;
      }
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.strId == -1 || Information.IsNothing( self.stringy))
        return windowReturnClass1;
      if (nr == 32 & self.detailx > -1 & self.editId > 0)
      {
        windowReturnClass2: WindowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.editId)] + 1, self.SubPartY[self.SubpartNr(self.editId)] + 1, 1);
        windowReturnClass2.SetFlag(true);
        return windowReturnClass2;
      }
      if (nr == 38 & self.detailx > 0)
      {
        --self.detailx;
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftUp();
        self.RefreshCellInfo();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 40 & self.detailx < self.stringy.Length)
      {
        this += 1.detailx;
        self.RefreshCellInfo();
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftDown();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 37 & self.detaily > 0)
      {
        --self.detaily;
        self.RefreshCellInfo();
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftLeft();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (!(nr == 39 & self.detaily < self.stringy.Width))
        return windowReturnClass1;
      this += 1.detaily;
      self.RefreshCellInfo();
      self.SubPartList[self.SubpartNr(self.tableId)].ShiftRight();
      self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
      windowReturnClass1.SetFlag(true);
      return windowReturnClass1;
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.tab1id)
            {
              self.tabId = 1;
              self.detailx = -1;
              self.detaily = -1;
              self.RemoveSubPart(self.tableId);
              self.tableId = 0;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.tab2id)
            {
              self.tabId = 2;
              self.detailx = -1;
              self.detaily = -1;
              self.RemoveSubPart(self.tableId);
              self.tableId = 0;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.tab3id)
            {
              self.tabId = 3;
              self.detailx = -1;
              self.detaily = -1;
              self.RemoveSubPart(self.tableId);
              self.tableId = 0;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.tab4id)
            {
              self.tabId = 4;
              self.detailx = -1;
              self.detaily = -1;
              self.RemoveSubPart(self.tableId);
              self.tableId = 0;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.loadId)
            {
              tinitdir: String = self.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( self.game.Data.ScenarioDir))
              {
                if (self.game.Data.ScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.Data.ScenarioDir);
                else if (self.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              }
              else if (self.game.ModScenarioDir.Length > 1)
                tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Historical library(*.se1his)|*.se1his", "Pick a historical library...", tinitdir, false);
              if (File.Exists(str))
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.EditObj.TutMode = false;
                if (self.game.Data.UseAI == 1)
                  self.game.NewAIObj.LastRegime = -1;
                self.game.SelectX = -1;
                self.game.SelectY = -1;
                self.game.Data = DataClass::new();
                GC.Collect();
                Application.DoEvents();
                self.game.HandyFunctionsObj.Unzip(str);
                self.game.Data = DataClass.deserialize(str);
                self.game.HandyFunctionsObj.ZipFile(str);
                if (Interaction.MsgBox( "Update with masterfile?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                {
                  self.game.Data.MasterfileReadPeople = false;
                  masterfileStart: String = self.masterfileStart;
                  filename: String = self.game.HandyFunctionsObj.ReturnLongMaster(str, masterfileStart);
                  self.game.EditObj.LoadString = "Loading Masterfile";
                  self.game.HandyFunctionsObj.LoadMasterFile(filename, alsohistorical: false, LoadVariants: false);
                }
                self.game.Data.MasterFile = "";
                self.game.Data.Round = 0;
                self.game.Data.Turn = 0;
                if ( self.game.Data.RuleVar[344] == 1.0 & self.game.EditObj.HideUnit == 0)
                  self.game.EditObj.HideUnit = 2;
                self.game.EditObj.TempValue = new MapMatrix2[self.game.Data.MapCounter + 1];
                self.game.EditObj.TempValue2 = new MapMatrix2[self.game.Data.MapCounter + 1];
                let mut mapCounter: i32 = self.game.Data.MapCounter;
                for (let mut index2: i32 = 0; index2 <= mapCounter; index2 += 1)
                {
                  self.game.EditObj.TempValue[index2] = new MapMatrix2(self.game.Data.MapObj[index2].MapWidth, self.game.Data.MapObj[index2].MapHeight);
                  self.game.EditObj.TempValue2[index2] = new MapMatrix2(self.game.Data.MapObj[index2].MapWidth, self.game.Data.MapObj[index2].MapHeight);
                }
                if (Strings.Len(self.game.Data.LoadPass) > 0)
                {
                  self.game.FormRef.Cursor = Cursors.Default;
                  if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by a load password. Please give it in order to load it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(self.game.Data.LoadPass), false) == 0)
                  {
                    let mut num2: i32 =  Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    let mut num3: i32 =  Interaction.MsgBox( "Wrong Password. You cannot Load this file. Exiting whole app.", Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.EndApp();
                  }
                }
                BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                self.game.Data.LoadGraphics((Form1) null);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                self.game.EditObj.StratMap = new Bitmap(self.game.ScreenWidth, self.game.ScreenHeight - 265);
                self.game.EditObj.StratMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.StratMap, self.game.ScreenWidth, self.game.ScreenHeight - 265, false, true, false);
                self.game.FormRef.Cursor = Cursors.Default;
                let mut num4: i32 =  Interaction.MsgBox( "Loaded Historical Library", Title: ( "Shadow Empire : Planetary Conquest"));
                self.detailx = -1;
                self.detaily = -1;
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.saveId)
            {
              self.game.Data.MasterFile = "";
              tinitdir: String = self.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( self.game.Data.ScenarioDir))
              {
                if (self.game.Data.ScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.Data.ScenarioDir);
                else if (self.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              }
              else if (self.game.ModScenarioDir.Length > 1)
                tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              str: String = self.game.HandyFunctionsObj.SaveSomething("SE1 Historical library(*.se1his)|*.se1his", "Give save name...", tinitdir, false);
              if (Strings.Len(str) < 2)
              {
                let mut num5: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.Interpret();
                self.game.Data.serialize(str);
                self.game.HandyFunctionsObj.ZipFile(str);
                windowReturnClass1.SetFlag(true);
                self.game.FormRef.Cursor = Cursors.Default;
                self.game.Data.LoadGraphics(self.formref);
                let mut num6: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            else
            {
              if (num1 == self.a1id)
              {
                str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].name);
                if (str.Length > 0)
                  self.game.Data.LibraryObj[0].name = str;
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (num1 == self.a2id)
              {
                str: String = Interaction.InputBox("Give author name.", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].creator);
                if (str.Length > 0)
                  self.game.Data.LibraryObj[0].creator = str;
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (num1 == self.a3id)
              {
                Form2::new( self.formref).Initialize(self.game.Data, 13, 0);
                self.Change = true;
                return windowReturnClass1;
              }
              tnr2_1: i32;
              if (num1 == self.exitId)
              {
                self.game.EditObj.InEditor = false;
                if (self.game.EditorBlock)
                  self.game.EditObj.ShowInitialMenu = true;
                if (self.game.ModIntroType == 0)
                  windowReturnClass1.AddCommand(3, 1);
                else
                  windowReturnClass1.AddCommand(3, 12);
              }
              else if (num1 == self.noneId)
              {
                if (self.tabId == 3)
                {
                  if (self.detaily <= 1)
                  {
                    let mut num7: i32 =  Interaction.MsgBox( "Cannot change", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    if (self.detaily == 4)
                    {
                      self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      self.stringy.Data[self.detailx, self.detaily] = "-1";
                      self.game.Data.HistoricalUnitObj[self.currentHisNr].SmallGfx = -1;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily >= 14)
                    {
                      self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut index3: i32 =  Math.Round( self.detaily / 2.0 - 7.0);
                      self.stringy.Data[self.detailx, self.detaily] = "-1";
                      self.game.Data.HistoricalUnitObj[self.currentHisNr].DesignationSmall[index3] = -1;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                  }
                }
              }
              else if (num1 == self.editId)
              {
                self.oldTopX = ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemX;
                self.oldTopY = ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemY;
                if (self.tabId == 1)
                {
                  if (self.detaily == 0)
                  {
                    let mut num8: i32 =  Interaction.MsgBox( "Changeing the ID# is not advised and for EXPERT USE ONLY. ", Title: ( "Shadow Empire : Planetary Conquest"));
                    if (Interaction.MsgBox( "Change ID# ?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    {
                      self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut num9: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new ID#", "Shadow Empire : Planetary Conquest", self.game.Data.PeopleObj[self.currentPplNr].id.ToString())));
                      self.stringy.Data[self.detailx, self.detaily] = num9.ToString();
                      self.game.Data.PeopleObj[self.currentPplNr].id = num9;
                      if (self.game.Data.PeopleIdCounter < num9)
                        self.game.Data.PeopleIdCounter = num9;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                  }
                  else
                  {
                    if (self.detaily == 1)
                    {
                      self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.game.Data.PeopleObj[self.currentPplNr].Name = str;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 2)
                    {
                      self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut num10: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give breakAt (0-100).", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily])));
                      if (num10 >= 0 & num10 <= 100)
                      {
                        self.stringy.Data[self.detailx, self.detaily] = num10.ToString();
                        self.game.Data.PeopleObj[self.currentPplNr].BreakAt = num10;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                      }
                      else
                      {
                        let mut num11: i32 =  Interaction.MsgBox( "no go. Between 0-100 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      return windowReturnClass1;
                    }
                    if (self.detaily == 7)
                    {
                      self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut num12: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give ArtCode. 0=default/none.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily])));
                      if (num12 >= 0 & num12 <= 100)
                      {
                        self.stringy.Data[self.detailx, self.detaily] = num12.ToString();
                        self.game.Data.PeopleObj[self.currentPplNr].artCode = num12;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                      }
                      else
                      {
                        let mut num13: i32 =  Interaction.MsgBox( "no go. Between 0-100 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      return windowReturnClass1;
                    }
                    if (self.detaily == 5)
                    {
                      self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut num14: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give BaseMorale (0-100).", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily])));
                      if (num14 >= 0 & num14 <= 100)
                      {
                        self.stringy.Data[self.detailx, self.detaily] = num14.ToString();
                        let mut index4: i32 = 0;
                        do
                        {
                          self.game.Data.PeopleObj[self.currentPplNr].BaseMorale[index4] = num14;
                          index4 += 1;
                        }
                        while (index4 <= 99);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                      }
                      else
                      {
                        let mut num15: i32 =  Interaction.MsgBox( "no go. Between 0-100 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      return windowReturnClass1;
                    }
                    if (self.detaily == 6)
                    {
                      self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      if (Interaction.MsgBox( "Use Color?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                      {
                        let mut num16: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Red (0-255).", "Shadow Empire : Planetary Conquest")));
                        if (num16 >= 0 & num16 <=  byte.MaxValue)
                        {
                          self.game.Data.PeopleObj[self.currentPplNr].Red = num16;
                        }
                        else
                        {
                          let mut num17: i32 =  Interaction.MsgBox( "no go. Between 0-255 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                        }
                        let mut num18: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Green (0-255).", "Shadow Empire : Planetary Conquest")));
                        if (num18 >= 0 & num18 <=  byte.MaxValue)
                        {
                          self.game.Data.PeopleObj[self.currentPplNr].Green = num18;
                        }
                        else
                        {
                          let mut num19: i32 =  Interaction.MsgBox( "no go. Between 0-255 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                        }
                        let mut num20: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Blue (0-255).", "Shadow Empire : Planetary Conquest")));
                        if (num20 >= 0 & num20 <=  byte.MaxValue)
                        {
                          self.game.Data.PeopleObj[self.currentPplNr].Blue = num20;
                        }
                        else
                        {
                          let mut num21: i32 =  Interaction.MsgBox( "no go. Between 0-255 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                        }
                      }
                      else
                      {
                        self.game.Data.PeopleObj[self.currentPplNr].Red = -1;
                        self.game.Data.PeopleObj[self.currentPplNr].Green = -1;
                        self.game.Data.PeopleObj[self.currentPplNr].Blue = -1;
                      }
                      str: String;
                      if (self.game.Data.PeopleObj[self.currentPplNr].Red > -1)
                        str = self.game.Data.PeopleObj[self.currentPplNr].Red.ToString() + "," + self.game.Data.PeopleObj[self.currentPplNr].Green.ToString() + "," + self.game.Data.PeopleObj[self.currentPplNr].Blue.ToString();
                      else
                        str = "No color";
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 3)
                    {
                      str: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of symbol Sprite:", self.game.AppPath + "graphics\\", true);
                      if (File.Exists(self.game.AppPath + "graphics/" + str))
                      {
                        self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.PeopleObj[self.currentPplNr].SidewaysFileName = str;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      let mut num22: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else if (self.detaily == 4)
                    {
                      String1: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of sideways Sprite:", self.game.AppPath + "graphics\\", true);
                      if (File.Exists(self.game.AppPath + "graphics/" + String1))
                      {
                        if (Strings.InStr(String1, "BIG") > 0 | Strings.InStr(String1, "SMALL") > 0)
                        {
                          let mut num23: i32 =  Interaction.MsgBox( "You cannot load a file that in the path include 'BIG' or 'SMALL'.", Title: ( "Shadow Empire : Planetary Conquest"));
                        }
                        else
                        {
                          self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                          self.stringy.Data[self.detailx, self.detaily] = String1;
                          self.game.Data.PeopleObj[self.currentPplNr].NationalFileName = String1;
                          self.RemoveSubPart(self.tableId);
                          self.tableId = 0;
                          self.DoStuff();
                          windowReturnClass1.SetFlag(true);
                          return windowReturnClass1;
                        }
                      }
                      else
                      {
                        let mut num24: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                    }
                  }
                }
                if (self.tabId == 2)
                {
                  if (self.detaily <= 1)
                  {
                    let mut num25: i32 =  Interaction.MsgBox( "Cannot change", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    if (self.detaily == 2)
                    {
                      self.currentUnitNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.game.Data.UnitObj[self.currentUnitNr].Name = str;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 3)
                    {
                      self.currentUnitNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      if (Interaction.MsgBox( "Is HQ?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                        self.game.Data.UnitObj[self.currentUnitNr].IsHQ = true;
                      else
                        self.game.Data.UnitObj[self.currentUnitNr].IsHQ = false;
                      self.stringy.Data[self.detailx, self.detaily] = self.game.Data.UnitObj[self.currentUnitNr].IsHQ.ToString();
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if ((self.detaily - 4 + 5) % 5 == 0)
                    {
                      self.currentUnitNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut index5: i32 =  Math.Round(Conversion.Int( (self.detaily - 4) / 5.0));
                      while (index5 > self.game.Data.UnitObj[self.currentUnitNr].SFCount)
                      {
                        self.game.Data.AddSF(self.currentUnitNr);
                        self.game.Data.SFObj[self.game.Data.SFCounter].People = -1;
                      }
                      self.Change = true;
                      Form3::new( self.formref).Initialize(self.game.Data, 131, self.game.Data.UnitObj[self.currentUnitNr].SFList[index5], tGame: self.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if ((self.detaily - 4 + 5) % 5 == 1)
                    {
                      self.currentUnitNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut index6: i32 =  Math.Round(Conversion.Int( (self.detaily - 4) / 5.0));
                      while (index6 > self.game.Data.UnitObj[self.currentUnitNr].SFCount)
                      {
                        self.game.Data.AddSF(self.currentUnitNr);
                        self.game.Data.SFObj[self.game.Data.SFCounter].People = -1;
                      }
                      self.Change = true;
                      Form3::new( self.formref).Initialize(self.game.Data, 132, self.game.Data.UnitObj[self.currentUnitNr].SFList[index6], tGame: self.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if ((self.detaily - 4 + 5) % 5 == 2)
                    {
                      self.currentUnitNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut index7: i32 =  Math.Round(Conversion.Int( (self.detaily - 4) / 5.0));
                      while (index7 > self.game.Data.UnitObj[self.currentUnitNr].SFCount)
                      {
                        self.game.Data.AddSF(self.currentUnitNr);
                        self.game.Data.SFObj[self.game.Data.SFCounter].People = -1;
                      }
                      let mut sf: i32 = self.game.Data.UnitObj[self.currentUnitNr].SFList[index7];
                      let mut num26: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Qty (0-999).", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily])));
                      if (num26 >= 0 & num26 <= 999)
                      {
                        self.stringy.Data[self.detailx, self.detaily] = num26.ToString();
                        self.game.Data.SFObj[sf].Qty = num26;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                      }
                      else
                      {
                        let mut num27: i32 =  Interaction.MsgBox( "no go. Between 0-999 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      return windowReturnClass1;
                    }
                    if ((self.detaily - 4 + 5) % 5 == 3)
                    {
                      self.currentUnitNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut index8: i32 =  Math.Round(Conversion.Int( (self.detaily - 4) / 5.0));
                      while (index8 > self.game.Data.UnitObj[self.currentUnitNr].SFCount)
                      {
                        self.game.Data.AddSF(self.currentUnitNr);
                        self.game.Data.SFObj[self.game.Data.SFCounter].People = -1;
                      }
                      let mut sf: i32 = self.game.Data.UnitObj[self.currentUnitNr].SFList[index8];
                      let mut num28: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Mor (0-100).", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily])));
                      if (num28 >= 0 & num28 <= 100)
                      {
                        self.stringy.Data[self.detailx, self.detaily] = num28.ToString();
                        self.game.Data.SFObj[sf].Mor = num28;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                      }
                      else
                      {
                        let mut num29: i32 =  Interaction.MsgBox( "no go. Between 0-100 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      return windowReturnClass1;
                    }
                    if ((self.detaily - 4 + 5) % 5 == 4)
                    {
                      self.currentUnitNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      let mut index9: i32 =  Math.Round(Conversion.Int( (self.detaily - 4) / 5.0));
                      while (index9 > self.game.Data.UnitObj[self.currentUnitNr].SFCount)
                      {
                        self.game.Data.AddSF(self.currentUnitNr);
                        self.game.Data.SFObj[self.game.Data.SFCounter].People = -1;
                      }
                      let mut sf: i32 = self.game.Data.UnitObj[self.currentUnitNr].SFList[index9];
                      let mut num30: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give XP (0-100).", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily])));
                      if (num30 >= 0 & num30 <= 100)
                      {
                        self.stringy.Data[self.detailx, self.detaily] = num30.ToString();
                        self.game.Data.SFObj[sf].Xp = num30;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                      }
                      else
                      {
                        let mut num31: i32 =  Interaction.MsgBox( "no go. Between 0-100 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      return windowReturnClass1;
                    }
                  }
                }
                if (self.tabId == 3)
                {
                  if (self.detaily <= 1)
                  {
                    let mut num32: i32 =  Interaction.MsgBox( "Cannot change", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    if (self.detaily == 2)
                    {
                      self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.game.Data.HistoricalUnitObj[self.currentHisNr].Name = str;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 3)
                    {
                      self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      self.Change = true;
                      Form3::new( self.formref).Initialize(self.game.Data, 133, self.currentHisNr, tGame: self.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 4)
                    {
                      str: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Small Gfx:", self.game.AppPath + "graphics\\", true);
                      if (Information.IsNothing( str))
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.stringy.TempBmp[self.detailx, self.detaily] = 0;
                        self.stringy.Data[self.detailx, self.detaily] = "";
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].SmallGfx = -1;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (File.Exists(self.game.AppPath + "graphics/" + str))
                      {
                        if (Strings.InStr(str, "BIG") > 0 | Strings.InStr(str, "SMALL") > 0)
                        {
                          let mut num33: i32 =  Interaction.MsgBox( "You cannot load a file that in the path include 'BIG' or 'SMALL'.", Title: ( "Shadow Empire : Planetary Conquest"));
                        }
                        else
                        {
                          let mut index10: i32 = self.game.Data.FindSmallPic(str, -1, self.game.Data.LibraryObj[0].name);
                          if (index10 == -1)
                          {
                            self.game.Data.AddSmallPic(str);
                            self.game.Data.SmallLibId[self.game.Data.SmallPicCounter].libSlot = 0;
                            self.game.Data.SmallLibId[self.game.Data.SmallPicCounter].id = -1;
                            index10 = self.game.Data.SmallPicCounter;
                          }
                          self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                          self.stringy.TempBmp[self.detailx, self.detaily] = self.game.Data.SmallPicNr[index10];
                          self.stringy.Data[self.detailx, self.detaily] = index10.ToString();
                          self.game.Data.HistoricalUnitObj[self.currentHisNr].SmallGfx = index10;
                          self.RemoveSubPart(self.tableId);
                          self.tableId = 0;
                          self.DoStuff();
                          windowReturnClass1.SetFlag(true);
                          return windowReturnClass1;
                        }
                      }
                      else
                      {
                        let mut num34: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                    }
                    else
                    {
                      if (self.detaily == 5)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        str: String = Interaction.InputBox("Give shortname.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].CounterString = str;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 6)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        InputStr: String = Interaction.InputBox("Give Political Points.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = InputStr;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].PP =  Math.Round(Conversion.Val(InputStr));
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 7)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.Change = true;
                        Form3::new( self.formref).Initialize(self.game.Data, 134, self.currentHisNr, tGame: self.game);
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 8)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        str: String = Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].NameCounter = Conversions.ToInteger(str);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 9)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        if (Interaction.MsgBox( "Use Roman Numerals?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                        {
                          self.stringy.Data[self.detailx, self.detaily] = "True";
                          self.game.Data.HistoricalUnitObj[self.currentHisNr].UseRomans = true;
                        }
                        else
                        {
                          self.stringy.Data[self.detailx, self.detaily] = "False";
                          self.game.Data.HistoricalUnitObj[self.currentHisNr].UseRomans = false;
                        }
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 10)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.Change = true;
                        Form3::new( self.formref).Initialize(self.game.Data, 135, self.currentHisNr, tGame: self.game);
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 11)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.Change = true;
                        Form3::new( self.formref).Initialize(self.game.Data, 137, self.currentHisNr, tnr2_1, self.game);
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 12)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        str: String = Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].MaxPresent = Conversions.ToInteger(str);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 13)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        str: String = Interaction.InputBox("Give value (0=no,1=yes)", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].BattleGroup = Conversions.ToInteger(str);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily >= 14 & self.detaily % 2 == 0)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        let mut tnr2_2: i32 =  Math.Round(Conversion.Int( (self.detaily - 13) / 2.0));
                        self.Change = true;
                        Form3::new( self.formref).Initialize(self.game.Data, 136, self.currentHisNr, tnr2_2, self.game);
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily >= 15 & self.detaily % 2 == 1)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        let mut index11: i32 =  Math.Round(Conversion.Int( (self.detaily - 13) / 2.0)) - 1;
                        str: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Small Gfx", self.game.AppPath + "graphics\\", true);
                        if (!Information.IsNothing( str))
                        {
                          if (File.Exists(self.game.AppPath + "graphics/" + str))
                          {
                            if (Strings.InStr(str, "BIG") > 0 | Strings.InStr(str, "SMALL") > 0)
                            {
                              let mut num35: i32 =  Interaction.MsgBox( "You cannot load a file that in the path include 'BIG' or 'SMALL'.", Title: ( "Shadow Empire : Planetary Conquest"));
                            }
                            else
                            {
                              let mut index12: i32 = self.game.Data.FindSmallPic(str, -1, self.game.Data.LibraryObj[0].name);
                              if (index12 == -1)
                              {
                                self.game.Data.AddSmallPic(str);
                                self.game.Data.SmallLibId[self.game.Data.SmallPicCounter].libSlot = 0;
                                self.game.Data.SmallLibId[self.game.Data.SmallPicCounter].id = -1;
                                index12 = self.game.Data.SmallPicCounter;
                              }
                              self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                              self.stringy.TempBmp[self.detailx, self.detaily] = self.game.Data.SmallPicNr[index12];
                              self.stringy.Data[self.detailx, self.detaily] = index12.ToString();
                              self.game.Data.HistoricalUnitObj[self.currentHisNr].DesignationSmall[index11] = index12;
                              self.RemoveSubPart(self.tableId);
                              self.tableId = 0;
                              self.DoStuff();
                              windowReturnClass1.SetFlag(true);
                              return windowReturnClass1;
                            }
                          }
                          else
                          {
                            let mut num36: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                          }
                        }
                      }
                    }
                  }
                }
                if (self.tabId == 4)
                {
                  if (self.detaily <= 1)
                  {
                    let mut num37: i32 =  Interaction.MsgBox( "Cannot change", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    if (self.detaily == 2)
                    {
                      self.currentInstNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.game.Data.HistoricalUnitObj[self.currentInstNr].Name = str;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 3)
                    {
                      self.currentInstNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      self.Change = true;
                      Form3::new( self.formref).Initialize(self.game.Data, 138, self.currentInstNr, tGame: self.game);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 4)
                    {
                      self.currentInstNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Interaction.InputBox("Give shortname", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.game.Data.HistoricalUnitObj[self.currentInstNr].CounterString = str;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily >= 5)
                    {
                      self.currentInstNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      if (self.detaily == 5)
                        self.game.Data.HistoricalUnitObj[self.currentInstNr].TempVar1 = Conversions.ToInteger(str);
                      if (self.detaily == 6)
                        self.game.Data.HistoricalUnitObj[self.currentInstNr].TempVar2 = Conversions.ToInteger(str);
                      if (self.detaily == 7)
                        self.game.Data.HistoricalUnitObj[self.currentInstNr].TempVar3 = Conversions.ToInteger(str);
                      if (self.detaily == 8)
                        self.game.Data.HistoricalUnitObj[self.currentInstNr].TempVar4 = Conversions.ToInteger(str);
                      if (self.detaily == 9)
                        self.game.Data.HistoricalUnitObj[self.currentInstNr].TempVar5 = Conversions.ToInteger(str);
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                  }
                  return windowReturnClass1;
                }
              }
              else
              {
                if (num1 == self.removeId)
                {
                  if (self.tabId == 1)
                  {
                    self.currentPplNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                    --self.detailx;
                    self.game.Data.RemovePeople(self.currentPplNr);
                  }
                  else if (self.tabId == 2)
                  {
                    self.currentUnitNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                    --self.detailx;
                    data: DataClass = self.game.Data;
                    let mut currentUnitNr: i32 = self.currentUnitNr;
                    let mut gameClass: GameClass = (GameClass) null;
                     let mut local: GameClass =  gameClass;
                    data.RemoveUnit(currentUnitNr,  local);
                  }
                  else if (self.tabId == 3)
                  {
                    self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                    --self.detailx;
                    self.game.Data.RemoveHistoricalUnit(self.currentHisNr);
                  }
                  else if (self.tabId == 4)
                  {
                    self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                    --self.detailx;
                    self.game.Data.RemoveHistoricalUnit(self.currentHisNr);
                  }
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.addId)
                {
                  if (self.tabId == 3 | self.tabId == 4)
                  {
                    let mut num38: i32 = 0;
                    let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
                    for (let mut index13: i32 = 0; index13 <= historicalUnitCounter; index13 += 1)
                    {
                      if (self.game.Data.HistoricalUnitObj[index13].ID >= num38)
                        num38 = self.game.Data.HistoricalUnitObj[index13].ID;
                    }
                    if (num38 + 100 < self.game.Data.HistoricalIDCounter)
                      self.game.Data.HistoricalIDCounter = num38 + 100;
                  }
                  if (self.tabId == 1)
                  {
                    self.game.Data.AddPeople();
                    self.game.Data.PeopleObj[self.game.Data.PeopleCounter].LibId.libSlot = 0;
                  }
                  else if (self.tabId == 2)
                  {
                    tnr2_1 = self.game.Data.AddUnit(0, 0, 0);
                    let mut unitCounter: i32 = self.game.Data.UnitCounter;
                    self.game.Data.UnitObj[unitCounter].Regime = 0;
                    self.game.Data.MapObj[0].HexObj[0, 0].RemoveUnitFromList(unitCounter);
                    self.game.Data.UnitObj[unitCounter].PreDef = self.game.HandyFunctionsObj.GetNextPreDefNr();
                    self.game.Data.UnitObj[unitCounter].LibId.libSlot = 0;
                  }
                  else if (self.tabId == 3)
                  {
                    self.game.Data.AddHistoricalUnit();
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].Model = true;
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].TempRegime = 0;
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].LibId.libSlot = 0;
                  }
                  else if (self.tabId == 4)
                  {
                    self.game.Data.AddHistoricalUnit();
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].Model = false;
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].TempRegime = 0;
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].LibId.libSlot = 0;
                  }
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.versionid)
                {
                  InputStr: String = Interaction.InputBox("Give version.", "Shadow Empire : Planetary Conquest");
                  if (InputStr.Length > 0)
                    self.game.Data.LibraryObj[0].version =  Math.Round(Conversion.Val(InputStr));
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.removeReinf)
                {
                  self.game.Data.RemoveLibrary(self.detailnr);
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.detailnr = -1;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.addReinf)
                {
                  path: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Troops&Equipment library(*.se1troops)|*.se1troops", "Pick troopType library...", self.game.AppPath + self.game.ModScenarioDir, false);
                  if (File.Exists(path))
                  {
                    self.game.EditObj.TempFileName = path;
                    self.game.EditObj.TempFileType = NewEnums.LibFileType.ImportTroopsInHistoricalEditor;
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    dataClass: DataClass;
                    self.game.HandyFunctionsObj.LoadLibrary( dataClass);
                    bool[] import = new bool[dataClass.LibraryCounter + 1];
                    int[] subreg = new int[dataClass.RegimeCounter + 1];
                    int[] subPpl = new int[dataClass.PeopleCounter + 1];
                    int[] sublib = new int[dataClass.LibraryCounter + 1];
                    let mut libraryCounter1: i32 = dataClass.LibraryCounter;
                    for (let mut index14: i32 = 0; index14 <= libraryCounter1; index14 += 1)
                    {
                      sublib[index14] = -1;
                      import[index14] = false;
                    }
                    let mut regimeCounter: i32 = dataClass.RegimeCounter;
                    for (let mut index15: i32 = 0; index15 <= regimeCounter; index15 += 1)
                      subreg[index15] = 0;
                    let mut peopleCounter: i32 = dataClass.PeopleCounter;
                    for (let mut index16: i32 = 0; index16 <= peopleCounter; index16 += 1)
                      subPpl[index16] = -1;
                    let mut libraryCounter2: i32 = dataClass.LibraryCounter;
                    for (let mut index17: i32 = 0; index17 <= libraryCounter2; index17 += 1)
                    {
                      sublib[index17] = -1;
                      let mut libraryCounter3: i32 = self.game.Data.LibraryCounter;
                      for (let mut index18: i32 = 0; index18 <= libraryCounter3; index18 += 1)
                      {
                        if (Operators.CompareString(self.game.Data.LibraryObj[index18].name, dataClass.LibraryObj[index17].name, false) == 0)
                        {
                          bool flag;
                          sublib[-(flag ? 1 : 0)] = index18;
                        }
                      }
                      let mut sfTypeCounter: i32 = dataClass.SFTypeCounter;
                      for (let mut index19: i32 = 0; index19 <= sfTypeCounter; index19 += 1)
                      {
                        if (dataClass.SFTypeObj[index19].LibId.libSlot == index17)
                          import[index17] = true;
                      }
                    }
                    self.game.HandyFunctionsObj.ActuallyImportLibs( dataClass,  import,  sublib,  subPpl,  subreg);
                    let mut num39: i32 =  Interaction.MsgBox( "Import completed", Title: ( "Shadow Empire : Planetary Conquest"));
                    self.game.FormRef.Cursor = Cursors.Default;
                    self.RemoveSubPart(self.tableId);
                    self.tableId = 0;
                    self.DoStuff();
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  let mut num40: i32 =  Interaction.MsgBox( "Could not find file", Title: ( "Shadow Empire : Planetary Conquest"));
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.listId)
                {
                  let mut num41: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (num41 > -1)
                  {
                    self.detailnr = num41;
                    self.RemoveSubPart(self.tableId);
                    self.tableId = 0;
                    self.DoStuff();
                  }
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.tableId)
                {
                  Coordinate coordinate = self.SubPartList[index1].Click2(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (coordinate.x > -1)
                  {
                    self.detailx = coordinate.x;
                    self.detaily = coordinate.y;
                    if (self.detaily > self.stringy.Width)
                      self.detaily = self.stringy.Width;
                    if (self.detailx > self.stringy.Length)
                      self.detailx = self.stringy.Length;
                  }
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.importCsv)
                {
                  str1: String = self.game.HandyFunctionsObj.LoadSomething("CSV (*.csv)|*.csv|TXT (*.txt)|*.txt|All (*.*)|*.*", "Select file to import...", self.game.AppPath + "csv/", false);
                  if (Strings.Len(str1) < 2)
                  {
                    let mut num42: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    StreamReader Expression;
                    windowReturnClass2: WindowReturnClass;
                    try
                    {
                      Expression = File.OpenText(str1);
                      let mut num43: i32 = 0;
                      str2: String = ",";
                      while (!Expression.EndOfStream)
                      {
                        str3: String = Expression.ReadLine();
                        if ((uint) Strings.InStr(str3, "sep=") > 0U)
                        {
                          str2 = Strings.Mid(str3, 5, 1);
                          num43 = 1;
                        }
                        else
                        {
                          switch (num43)
                          {
                            case 0:
                              if (Strings.InStr(str3, "\t") > 0)
                                str2 = "\t";
                              else if (Strings.InStr(str3, ",") > 0)
                                str2 = ",";
                              else if (Strings.InStr(str3, ";") > 0)
                                str2 = ";";
                              num43 = 2;
                              continue;
                            case 1:
                              num43 = 2;
                              continue;
                            case 2:
                              strArray: Vec<String> = str3.Split(Conversions.ToChar(str2));
                              let mut id: i32 =  Math.Round(Conversion.Val(strArray[0]));
                              index20: i32;
                              if (id > 0)
                              {
                                index20 = self.game.HandyFunctionsObj.GetHistoricalUnitByID(id);
                                if (index20 == -1)
                                {
                                  self.game.Data.AddHistoricalUnit();
                                  index20 = self.game.Data.HistoricalUnitCounter;
                                  self.game.Data.HistoricalUnitObj[index20].ID = Conversions.ToInteger(strArray[0]);
                                }
                                else if (self.game.Data.HistoricalUnitObj[index20].Model)
                                {
                                  self.game.Data.AddHistoricalUnit();
                                  index20 = self.game.Data.HistoricalUnitCounter;
                                  strArray[0] = Conversions.ToString(self.game.Data.HistoricalUnitObj[index20].ID);
                                }
                              }
                              else
                              {
                                self.game.Data.AddHistoricalUnit();
                                index20 = self.game.Data.HistoricalUnitCounter;
                                strArray[0] = Conversions.ToString(self.game.Data.HistoricalUnitObj[index20].ID);
                              }
                              self.game.Data.HistoricalUnitObj[index20].ID =  Math.Round(Conversion.Val(strArray[0]));
                              self.game.Data.HistoricalUnitObj[index20].Name = strArray[1];
                              self.game.Data.HistoricalUnitObj[index20].ModelMaster = self.game.HandyFunctionsObj.GetHistoricalUnitByID( Math.Round(Conversion.Val(strArray[2])));
                              self.game.Data.HistoricalUnitObj[index20].CounterString = strArray[3];
                              self.game.Data.HistoricalUnitObj[index20].TempVar1 =  Math.Round(Conversion.Val(strArray[4]));
                              self.game.Data.HistoricalUnitObj[index20].TempVar2 =  Math.Round(Conversion.Val(strArray[5]));
                              self.game.Data.HistoricalUnitObj[index20].TempVar3 =  Math.Round(Conversion.Val(strArray[6]));
                              self.game.Data.HistoricalUnitObj[index20].TempVar4 =  Math.Round(Conversion.Val(strArray[7]));
                              self.game.Data.HistoricalUnitObj[index20].TempVar5 =  Math.Round(Conversion.Val(strArray[8]));
                              continue;
                            default:
                              continue;
                          }
                        }
                      }
                    }
                    catch (Exception ex)
                    {
                      ProjectData.SetProjectError(ex);
                      Exception exception = ex;
                      if (!Information.IsNothing( Expression))
                        Expression.Close();
                      let mut num44: i32 =  Interaction.MsgBox( ("Import problem encountered: '" + exception.Message + "'. Your current data might be corrupted. "), Title: ( "Shadow Empire : Planetary Conquest"));
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      windowReturnClass2 = windowReturnClass1;
                      ProjectData.ClearProjectError();
                      goto label_350;
                    }
                    Expression.Close();
                    let mut num45: i32 =  Interaction.MsgBox( "Import finished", Title: ( "Shadow Empire : Planetary Conquest"));
                    self.RemoveSubPart(self.tableId);
                    self.tableId = 0;
                    self.DoStuff();
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
label_350:
                    return windowReturnClass2;
                  }
                }
                else if (num1 == self.exportCsv)
                {
                  str4: String = self.game.HandyFunctionsObj.SaveSomething("CSV (*.csv)|*.csv|TXT (*.txt)|*.txt|All (*.*)|*.*", "Give save name...", self.game.AppPath + "csv/", false);
                  if (Strings.Len(str4) < 2)
                  {
                    let mut num46: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    try
                    {
                      StreamWriter text = File.CreateText(str4);
                      try
                      {
                        text.WriteLine("sep=\t");
                        str5: String = "" + "ID#" + "\t" + "Name" + "\t" + "Based On Model" + "\t" + "Shortname" + "\t" + "tv1" + "\t" + "tv2" + "\t" + "tv3" + "\t" + "tv4" + "\t" + "tv5";
                        text.WriteLine(str5);
                        let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
                        for (let mut index21: i32 = 0; index21 <= historicalUnitCounter; index21 += 1)
                        {
                          if (!self.game.Data.HistoricalUnitObj[index21].Model)
                          {
                            str6: String = "" + self.game.Data.HistoricalUnitObj[index21].ID.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index21].Name + "\t";
                            str7: String = (self.game.Data.HistoricalUnitObj[index21].ModelMaster <= -1 ? str6 + "-1" : str6 + self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitObj[index21].ModelMaster].ID.ToString()) + "\t" + self.game.Data.HistoricalUnitObj[index21].CounterString + "\t" + self.game.Data.HistoricalUnitObj[index21].TempVar1.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index21].TempVar2.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index21].TempVar3.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index21].TempVar4.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index21].TempVar5.ToString();
                            text.WriteLine(str7);
                          }
                        }
                        text.Close();
                        let mut num47: i32 =  Interaction.MsgBox( "Export has been written to the csv/ directory", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      catch (Exception ex)
                      {
                        ProjectData.SetProjectError(ex);
                        Exception exception = ex;
                        text.Close();
                        let mut num48: i32 =  Interaction.MsgBox( ("Problem writing: " + exception.Message), Title: ( "Shadow Empire : Planetary Conquest"));
                        ProjectData.ClearProjectError();
                      }
                    }
                    catch (Exception ex)
                    {
                      ProjectData.SetProjectError(ex);
                      let mut num49: i32 =  Interaction.MsgBox( "Problem writing. Check if the file is not opened in other application please.", Title: ( "Shadow Empire : Planetary Conquest"));
                      ProjectData.ClearProjectError();
                    }
                  }
                }
              }
            }
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub fn Interpret()
    {
      bool[] flagArray = new bool[self.game.Data.SmallPicCounter + 1];
      let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
      for (let mut index1: i32 = 0; index1 <= historicalUnitCounter; index1 += 1)
      {
        if (self.game.Data.HistoricalUnitObj[index1].LibId.libSlot == 0)
        {
          self.game.Data.HistoricalUnitObj[index1].NameCounterBackup = self.game.Data.HistoricalUnitObj[index1].NameCounter;
          if (self.game.Data.HistoricalUnitObj[index1].SmallGfx > -1)
          {
            if (self.game.Data.HistoricalUnitObj[index1].SmallGfx == 110)
              index1 = index1;
            flagArray[self.game.Data.HistoricalUnitObj[index1].SmallGfx] = true;
          }
          let mut index2: i32 = 0;
          do
          {
            if (self.game.Data.HistoricalUnitObj[index1].DesignationSmall[index2] > -1)
              flagArray[self.game.Data.HistoricalUnitObj[index1].DesignationSmall[index2]] = true;
            index2 += 1;
          }
          while (index2 <= 5);
        }
      }
      for (let mut smallPicCounter: i32 = self.game.Data.SmallPicCounter; smallPicCounter >= 0; smallPicCounter += -1)
      {
        if (self.game.Data.SmallLibId[smallPicCounter].libSlot == 0 && !flagArray[smallPicCounter])
          self.game.Data.RemoveSmallPic(smallPicCounter);
      }
    }
  }
}
