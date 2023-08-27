// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleEditTableWindowClass
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
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleEditTableWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     ListClass libListObj;
     libTableId: i32;
     tableId: i32;
     text1id: i32;
     addId: i32;
     remId: i32;
     remIdb: i32;
     editId: i32;
     editidb: i32;
     importCsv: i32;
     exportCsv: i32;
     strId: i32;
     detailx: i32;
     detaily: i32;
     cellinfoid: i32;
     libSelect: i32;
     switchId: i32;
     fullForwardId: i32;
     forwardId: i32;
     backwardId: i32;
     fullBackwardId: i32;
     mode: i32;
     pageNumberId: i32;

    pub SimpleEditTableWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 50, 9, tDoBorders: 1, tHeaderString: "Tables")
    {
      self.strId = -1;
      self.detailx = -1;
      self.detaily = -1;
      self.libSelect = -1;
      self.mode = 1;
      self.DoStuff();
    }

    pub fn RefreshCellInfo()
    {
      if (self.cellinfoid > 0)
      {
        self.RemoveSubPart(self.cellinfoid);
        self.cellinfoid = 0;
      }
      txt: String;
      if (self.detaily == -1 | self.detailx == -1)
        txt = "No cell selected";
      else if (self.strId > -1)
        txt = "(row" + self.detailx.ToString() + ",col" + self.detaily.ToString() + ")             " + self.game.Data.StringListObj[self.strId].ColumnName[self.detaily].ToUpper() + "                " + self.game.Data.StringListObj[self.strId].Data[self.detailx, self.detaily];
      let mut tsubpart: SubPartClass =  TextPartClass::new(txt, self.game.MarcFont4, self.game.ScreenWidth - 23, 20, false, tMarc: true);
      self.cellinfoid = self.AddSubPart( tsubpart, 12, 49, self.game.ScreenWidth - 32, 20, 0);
    }

    pub fn DoRefresh() => self.DoStuff();

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.strId == -1)
        return windowReturnClass1;
      if ((nr == 32 | nr == 13) & self.detailx > -1 & self.editId > 0)
      {
        windowReturnClass2: WindowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.editId)] + 1, self.SubPartY[self.SubpartNr(self.editId)] + 1, 1);
        windowReturnClass2.SetFlag(true);
        return windowReturnClass2;
      }
      if (nr == 38 & self.detailx > 0)
      {
        --self.detailx;
        self.RefreshCellInfo();
        let mut index: i32 = self.SubpartNr(self.tableId);
        if (index > -1)
          self.SubPartFlag[index] = true;
        if (index > -1)
          self.SubPartList[index].Refresh(self.game.Data.StringListObj[self.strId], self.detailx, self.detaily);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 40 & self.detailx < self.game.Data.StringListObj[self.strId].Length)
      {
        this += 1.detailx;
        self.RefreshCellInfo();
        let mut index: i32 = self.SubpartNr(self.tableId);
        if (index > -1)
          self.SubPartFlag[index] = true;
        if (index > -1)
          self.SubPartList[index].Refresh(self.game.Data.StringListObj[self.strId], self.detailx, self.detaily);
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (self.mode == 1)
      {
        if (nr == 37 & self.detaily > 0)
        {
          --self.detaily;
          self.RefreshCellInfo();
          let mut index: i32 = self.SubpartNr(self.tableId);
          if (index > -1)
            self.SubPartFlag[index] = true;
          if (index > -1)
            self.SubPartList[index].Refresh(self.game.Data.StringListObj[self.strId], self.detailx, self.detaily);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
        if (nr == 39 & self.detaily < self.game.Data.StringListObj[self.strId].Width)
        {
          this += 1.detaily;
          self.RefreshCellInfo();
          let mut index: i32 = self.SubpartNr(self.tableId);
          if (index > -1)
            self.SubPartFlag[index] = true;
          if (index > -1)
            self.SubPartList[index].Refresh(self.game.Data.StringListObj[self.strId], self.detailx, self.detaily);
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
      }
      return windowReturnClass1;
    }

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      if (self.addId > 0)
        self.RemoveSubPart(self.addId);
      if (self.remId > 0)
        self.RemoveSubPart(self.remId);
      if (self.remIdb > 0)
        self.RemoveSubPart(self.remIdb);
      if (self.editId > 0)
        self.RemoveSubPart(self.editId);
      if (self.editidb > 0)
        self.RemoveSubPart(self.editidb);
      if (self.tableId > 0)
        self.RemoveSubPart(self.tableId);
      if (self.importCsv > 0)
        self.RemoveSubPart(self.importCsv);
      if (self.exportCsv > 0)
        self.RemoveSubPart(self.exportCsv);
      if (self.text1id > 0)
        self.RemoveSubPart(self.text1id);
      if (self.listId > 0)
      {
        self.RemoveSubPart(self.listId);
        self.listId = 0;
      }
      if (self.libTableId > 0)
      {
        self.RemoveSubPart(self.libTableId);
        self.libTableId = 0;
      }
      if (self.cellinfoid > 0)
      {
        self.RemoveSubPart(self.cellinfoid);
        self.cellinfoid = 0;
      }
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objgraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objgraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objgraphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 50, -1);
      let mut num3: i32 = -1;
      num4: i32;
      if (num2 > 10)
        num4 =  Math.Round( (num2 - 10) * 0.33 / 16.0);
      self.libListObj = ListClass::new();
      self.libListObj.add("All", -2);
      if (self.libSelect == -1)
        num3 = 0;
      let mut num5: i32 = 0;
      if (num1 >= 256)
      {
        DrawMod.DrawTextColouredMarc( objgraphics, "Libraries:", self.game.MarcFont4, 10 + num1 - 256,  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 55.0 + 30.0), Color.White);
        let mut libraryCounter: i32 = self.game.Data.LibraryCounter;
        for (let mut index1: i32 = 0; index1 <= libraryCounter; index1 += 1)
        {
          bool flag = false;
          let mut stringListCounter: i32 = self.game.Data.StringListCounter;
          for (let mut index2: i32 = 0; index2 <= stringListCounter; index2 += 1)
          {
            if (self.game.Data.StringListObj[index2].LibId.libSlot == index1 && self.game.Data.StringListObj[index2].Editable)
            {
              flag = true;
              break;
            }
          }
          if (flag)
          {
            num5 += 1;
            self.libListObj.add(Conversion.Str( index1) + ") " + self.game.Data.LibraryObj[index1].name, index1);
            if (self.libSelect == index1)
              num3 = num5;
          }
        }
        if (self.libListObj.ListCount > 0)
        {
          ListClass libListObj = self.libListObj;
          let mut tlistsize: i32 = 9 + num4;
          let mut tlistselect: i32 = num3;
          let mut game: GameClass = self.game;
           local1: Bitmap =  self.OwnBitmap;
          let mut bbx: i32 = 10 + num1 - 256;
          let mut bby: i32 =  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 55.0 + 50.0);
          font: Font =  null;
           local2: Font =  font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(libListObj, tlistsize, 250, tlistselect, game, true, "Tables", tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx, bby: bby, tMarcStyle: true, overruleFont: ( local2));
          self.libTableId = self.AddSubPart( tsubpart, 10 + num1 - 256,  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 55.0 + 50.0), 250, (12 + num4) * 16, 0);
        }
      }
      tstring: String = "Tables:";
      DrawMod.DrawTextColouredMarc( objgraphics, tstring, self.game.MarcFont4, 10 + num1,  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 55.0 + 30.0), Color.White);
      let mut num6: i32 = -1;
      let mut num7: i32 = -1;
      self.listObj = ListClass::new();
      let mut stringListCounter1: i32 = self.game.Data.StringListCounter;
      for (let mut index: i32 = 0; index <= stringListCounter1; index += 1)
      {
        if (self.game.Data.StringListObj[index].Editable && self.game.Data.StringListObj[index].LibId.libSlot == self.libSelect | self.libSelect == -1)
        {
          num6 += 1;
          self.listObj.add(Conversion.Str( index) + ") " + self.game.Data.StringListObj[index].Name, index);
          if (num7 == -1)
            num7 = 0;
          if (self.strId == -1)
            self.strId = index;
          if (self.strId == index)
            num7 = num6;
        }
      }
      if (num7 > num6)
        num7 = 0;
      if (num7 == -1)
        self.strId = -1;
      SubPartClass tsubpart1;
      if (self.game.Data.StringListCounter > -1)
      {
        ListClass listObj = self.listObj;
        let mut tlistsize: i32 = 9 + num4;
        let mut tlistselect: i32 = num7;
        let mut game: GameClass = self.game;
         local3: Bitmap =  self.OwnBitmap;
        let mut bbx: i32 = 10 + num1;
        let mut bby: i32 =  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 55.0 + 50.0);
        font: Font =  null;
         local4: Font =  font;
        tsubpart1 =  new ListSubPartClass(listObj, tlistsize, 250, tlistselect, game, true, "Tables", tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local3), bbx: bbx, bby: bby, tMarcStyle: true, overruleFont: ( local4));
        self.listId = self.AddSubPart( tsubpart1, 10 + num1,  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 55.0 + 50.0), 250, (12 + num4) * 16, 0);
      }
      if (self.strId <= -1)
        return;
      if (self.mode == 1)
      {
        tsubpart1 =  new MatrixSubPartClass(self.game.Data.StringListObj[self.strId], 13 +  Math.Round( num2 * 0.66 / 24.0), Math.Max(1000, self.game.ScreenWidth - 24), self.detailx, self.detaily, self.game, tbackbitmap: ( self.BackBitmap), bbx: 10, bby: 72, trowheight: 24, tfontsize: 16, tfontoffsety: 4, tMarcy: true, tLibSlot: self.game.Data.StringListObj[self.strId].LibId.libSlot);
        self.tableId = self.AddSubPart( tsubpart1, 10, 72, Math.Max(1000, self.game.ScreenWidth - 24),  Math.Round((15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0), 0);
      }
      else
      {
        if (self.detailx > self.game.Data.StringListObj[self.strId].Length | self.detailx == -1 & self.game.Data.StringListObj[self.strId].Length > -1)
          self.detailx = 0;
        if (self.detailx > -1 & self.detailx <= self.game.Data.StringListObj[self.strId].Length)
        {
          StringListClass tListobj = new StringListClass(-9999);
          tListobj.AddCol(0, "Column Name");
          tListobj.AddCol(1, "Value");
          let mut width: i32 = self.game.Data.StringListObj[self.strId].Width;
          for (let mut col: i32 = 0; col <= width; col += 1)
          {
            LibVarClass libVarClass = self.game.Data.StringListObj[self.strId].GetValue( self.game.Data, self.detailx, col);
            tListobj.AddRowWithData(self.game.Data.StringListObj[self.strId].ColumnName[col], libVarClass.valueText);
          }
          tsubpart1 =  new MatrixSubPartClass(tListobj, 13 +  Math.Round( num2 * 0.66 / 24.0), Math.Max(1000, self.game.ScreenWidth - 24), self.detaily, 1, self.game, tbackbitmap: ( self.BackBitmap), bbx: 10, bby: 72, trowheight: 24, tfontsize: 16, tfontoffsety: 4, tMarcy: true, tTwoColumnVariant: 200);
          self.tableId = self.AddSubPart( tsubpart1, 10, 72, Math.Max(1000, self.game.ScreenWidth - 24),  Math.Round((15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0), 0);
        }
        else
          DrawMod.DrawTextColouredMarcCenter( objgraphics, "No rows in table", self.game.MarcFont4, num1 + 512, 256 +  Math.Round( num2 * 0.33), Color.White);
      }
      str1: String = " (Id " + self.game.Data.StringListObj[self.strId].ID.ToString();
      if (self.game.Data.StringListObj[self.strId].LibId.libSlot > -1)
        str1 = str1 + ", LibId " + self.game.Data.StringListObj[self.strId].ID.ToString();
      str2: String = str1 + ")";
      DrawMod.DrawTextColouredMarc( objgraphics, self.game.Data.StringListObj[self.strId].Name + str2 + " information:", self.game.MarcFont3, 310 + num1, 72 + (15 +  Math.Round( num2 * 0.66 / 24.0)) * 24 + 55 + 30, Color.White);
      tsubpart1 =  new TextAreaClass2(self.game,  Math.Round(Math.Max(700.0,  self.game.ScreenWidth / 2.0 - 324.0)), 4 +  Math.Round( num2 * 0.33 / 27.0), self.game.MarcFont4, self.game.Data.StringListObj[self.strId].Description, 27,  self.OwnBitmap, 310 + num1,  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 62.0 + 30.0));
      self.text1id = self.AddSubPart( tsubpart1, 310 + num1,  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 62.0 + 30.0),  Math.Round(Math.Max(700.0,  self.game.ScreenWidth / 2.0 - 324.0)), Math.Max(120, self.game.ScreenHeight - 648), 0);
      self.Tabby();
      if (!(self.detailx > -1 & self.detaily > -1 & !Information.IsNothing( self.tableId)))
        return;
      self.RefreshCellInfo();
    }

    pub fn Tabby()
    {
      if (self.addId > 0)
        self.RemoveSubPart(self.addId);
      if (self.remId > 0)
        self.RemoveSubPart(self.remId);
      if (self.remIdb > 0)
        self.RemoveSubPart(self.remIdb);
      if (self.editId > 0)
        self.RemoveSubPart(self.editId);
      if (self.editidb > 0)
        self.RemoveSubPart(self.editidb);
      if (self.importCsv > 0)
        self.RemoveSubPart(self.importCsv);
      if (self.exportCsv > 0)
        self.RemoveSubPart(self.exportCsv);
      if (self.switchId > 0)
        self.RemoveSubPart(self.switchId);
      if (self.forwardId > 0)
        self.RemoveSubPart(self.forwardId);
      if (self.backwardId > 0)
        self.RemoveSubPart(self.backwardId);
      if (self.fullForwardId > 0)
        self.RemoveSubPart(self.fullForwardId);
      if (self.fullBackwardId > 0)
        self.RemoveSubPart(self.fullBackwardId);
      if (self.pageNumberId > 0)
        self.RemoveSubPart(self.pageNumberId);
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      if (self.strId < 0)
        return;
      let mut num3: i32 =  Math.Round(72.0 + (15.0 +   Math.Round( num2 * 0.66) / 24.0) * 24.0 + 28.0);
      SubPartClass tsubpart;
      if (self.mode == 1)
      {
        tsubpart =  new TextButtonPartClass("=> Records", 150, tBackbitmap: ( self.OwnBitmap), bbx: (330 + num1), bby: num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.switchId = self.AddSubPart( tsubpart, 330 + num1, num3, 150, 35, 1);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("=> Table", 150, tBackbitmap: ( self.OwnBitmap), bbx: (330 + num1), bby: num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.switchId = self.AddSubPart( tsubpart, 330 + num1, num3, 150, 35, 1);
        tDescript: String = "";
        tsubpart =  new TextButtonPartClass("<<", 40, tDescript,  self.OwnBitmap, 10 + num1, num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.fullBackwardId = self.AddSubPart( tsubpart, 10 + num1, num3, 40, 35, 1);
        tsubpart =  new TextButtonPartClass("<", 40, tDescript,  self.OwnBitmap, 50 + num1, num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.backwardId = self.AddSubPart( tsubpart, 50 + num1, num3, 40, 35, 1);
        tsubpart =  new TextButtonPartClass(">", 40, tDescript,  self.OwnBitmap, 230 + num1, num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.forwardId = self.AddSubPart( tsubpart, 230 + num1, num3, 40, 35, 1);
        tsubpart =  new TextButtonPartClass(">>", 40, tDescript,  self.OwnBitmap, 270 + num1, num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.fullForwardId = self.AddSubPart( tsubpart, 270 + num1, num3, 40, 35, 1);
        txt: String = self.detailx.ToString() + " / " + self.game.Data.StringListObj[self.strId].Length.ToString();
        if (self.game.Data.StringListObj[self.strId].Length == -1)
          txt = "No rows";
        tsubpart =  TextPartClass::new(txt, self.game.MarcFont4, 120, 35, true, tBlackBack: true, tMarc: true);
        self.pageNumberId = self.AddSubPart( tsubpart, 100 + num1, num3, 120, 35, 0);
      }
      if (self.detailx > -1 & self.detaily > -1)
      {
        tsubpart =  new TextButtonPartClass("Edit cell", 150, tBackbitmap: ( self.OwnBitmap), bbx: (830 + num1), bby: num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editId = self.AddSubPart( tsubpart, 830 + num1, num3, 150, 35, 1);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("Edit cell", 150, "First select a cell you want to edit the value of",  self.OwnBitmap, 830 + num1, num3, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editidb = self.AddSubPart( tsubpart, 830 + num1, num3, 150, 35, 1);
      }
      tsubpart =  new TextButtonPartClass("Add row", 150, tBackbitmap: ( self.OwnBitmap), bbx: (510 + num1), bby: num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addId = self.AddSubPart( tsubpart, 510 + num1, num3, 150, 35, 1);
      if (self.detaily > -1)
      {
        tsubpart =  new TextButtonPartClass("Remove row", 150, tBackbitmap: ( self.OwnBitmap), bbx: (670 + num1), bby: num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.remId = self.AddSubPart( tsubpart, 670 + num1, num3, 150, 35, 1);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("Remove row", 150, "First select a row you want to remove",  self.OwnBitmap, 670 + num1, num3, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.remIdb = self.AddSubPart( tsubpart, 670 + num1, num3, 150, 35, 1);
      }
      if (self.mode != 1)
        return;
      tsubpart =  new TextButtonPartClass("Import Csv", 150, tBackbitmap: ( self.OwnBitmap), bbx: (10 + num1), bby: num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.importCsv = self.AddSubPart( tsubpart, 10 + num1, num3, 150, 35, 1);
      tsubpart =  new TextButtonPartClass("Export Csv", 150, tBackbitmap: ( self.OwnBitmap), bbx: (170 + num1), bby: num3, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.exportCsv = self.AddSubPart( tsubpart, 170 + num1, num3, 150, 35, 1);
    }

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
            if (num1 == self.editId)
            {
              if (self.detaily > -1 && self.game.Data.StringListObj[self.strId].LookUpCol[self.detaily] > 0)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 150, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.RegimeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 106, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.HistoricalUnitId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 144, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.HistoricalUnitModelId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 111, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.OfficerId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 112, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.LandscapeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 113, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.PeopleId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 114, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.RiverId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 115, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.LocationTypeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data,  sbyte.MaxValue, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.RoadId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 116, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.SFTypeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 145, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.ActionCardId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 146, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.SmallGfxId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 140, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.EventPicId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 141, self.strId, self.detailx, self.game, self.detaily);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.DateString)
              {
                if (self.game.Data.AlternateRound < 1)
                {
                  self.game.Data.StringListObj[self.strId].Data[self.detailx, self.detaily] = Interaction.InputBox("Give round number", "Shadow Empire : Planetary Conquest");
                }
                else
                {
                  str1: String = Interaction.InputBox("Give new Day.", "Shadow Empire : Planetary Conquest");
                  self.game.Data.StringListObj[self.strId].Data[self.detailx, self.detaily] = "";
                  if (Operators.CompareString(str1, "", false) != 0)
                  {
                    str2: String = Conversions.ToString(Conversion.Val(str1));
                    if (Conversions.ToInteger(str2) >= 1 & Conversions.ToInteger(str2) <= 31)
                    {
                      str3: String = str2;
                      str4: String = Interaction.InputBox("Give new Month.", "Shadow Empire : Planetary Conquest");
                      if (Operators.CompareString(str4, "", false) != 0)
                      {
                        str5: String = Conversions.ToString(Conversion.Val(str4));
                        if (Conversions.ToInteger(str5) >= 1 & Conversions.ToInteger(str5) <= 12)
                        {
                          str6: String = str3 + "/" + str5;
                          str7: String = Interaction.InputBox("Give new Year.", "Shadow Empire : Planetary Conquest");
                          if (Operators.CompareString(str7, "", false) != 0)
                          {
                            str8: String = Conversions.ToString(Conversion.Val(str7));
                            if (Conversions.ToInteger(str8) >= 1 & Conversions.ToInteger(str8) <= 9999)
                              self.game.Data.StringListObj[self.strId].Data[self.detailx, self.detaily] = str6 + "/" + str8;
                          }
                        }
                      }
                    }
                  }
                }
                if (self.mode == 2)
                  self.DoStuff();
                let mut index2: i32 = self.SubpartNr(self.tableId);
                if (index2 > -1)
                  self.SubPartFlag[index2] = true;
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.Data.StringListObj[self.strId].ColValueType[self.detaily] == NewEnums.LibVarValueType.YesNo)
              {
                if (Interaction.MsgBox( "Set value of cell to Yes or No?", MsgBoxStyle.YesNo,  "Cell value") == MsgBoxResult.Yes)
                  self.game.Data.StringListObj[self.strId].Data[self.detailx, self.detaily] = "1";
                else
                  self.game.Data.StringListObj[self.strId].Data[self.detailx, self.detaily] = "0";
                if (self.mode == 2)
                  self.DoStuff();
                let mut index3: i32 = self.SubpartNr(self.tableId);
                if (index3 > -1)
                  self.SubPartFlag[index3] = true;
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              self.game.Data.StringListObj[self.strId].Data[self.detailx, self.detaily] = Interaction.InputBox("Give new value of cell, please.", "Shadow Empire : Planetary Conquest", self.game.Data.StringListObj[self.strId].Data[self.detailx, self.detaily]);
              if (self.mode == 2)
                self.DoStuff();
              let mut index4: i32 = self.SubpartNr(self.tableId);
              if (index4 > -1)
                self.SubPartFlag[index4] = true;
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.remId)
            {
              self.game.Data.StringListObj[self.strId].RemoveRow(self.detailx);
              if (self.detailx > self.game.Data.StringListObj[self.strId].Length)
                self.detailx = self.game.Data.StringListObj[self.strId].Length;
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.addId)
            {
              index5: i32;
              if (self.detailx == -1)
              {
                self.game.Data.StringListObj[self.strId].AddRow(self.game.Data.StringListObj[self.strId].Length);
                index5 = self.game.Data.StringListObj[self.strId].Length;
              }
              else
              {
                self.game.Data.StringListObj[self.strId].AddRow(self.detailx);
                index5 = self.detailx + 1;
              }
              if (self.game.Data.Product == 7)
              {
                let mut width1: i32 = self.game.Data.StringListObj[self.strId].Width;
                for (let mut index6: i32 = 0; index6 <= width1; index6 += 1)
                {
                  if (self.game.Data.StringListObj[self.strId].SSID[index6] > 0 & self.game.Data.StringListObj[self.strId].SSID[index6] < 50)
                  {
                    let mut num2: i32 = 0;
                    let mut stringListCounter: i32 = self.game.Data.StringListCounter;
                    for (let mut index7: i32 = 0; index7 <= stringListCounter; index7 += 1)
                    {
                      if (self.game.Data.StringListObj[index7].Length > -1)
                      {
                        let mut width2: i32 = self.game.Data.StringListObj[index7].Width;
                        for (let mut index8: i32 = 0; index8 <= width2; index8 += 1)
                        {
                          if (self.game.Data.StringListObj[index7].SSID[index8] > 0 & self.game.Data.StringListObj[index7].SSID[index8] < 50)
                          {
                            let mut length: i32 = self.game.Data.StringListObj[index7].Length;
                            for (let mut index9: i32 = 0; index9 <= length; index9 += 1)
                            {
                              let mut num3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[index7].Data[index9, index8]));
                              if (num3 > num2)
                                num2 = num3;
                            }
                          }
                        }
                      }
                    }
                    num2 += 1;
                    self.game.Data.StringListObj[self.strId].Data[index5, index6] = num2.ToString();
                  }
                }
              }
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.forwardId)
            {
              this += 1.detailx;
              if (self.detailx > self.game.Data.StringListObj[self.strId].Length)
                self.detailx = self.game.Data.StringListObj[self.strId].Length;
              self.Tabby();
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.backwardId)
            {
              --self.detailx;
              if (self.detailx < 0)
                self.detailx = 0;
              if (self.detailx > self.game.Data.StringListObj[self.strId].Length)
                self.detailx = self.game.Data.StringListObj[self.strId].Length;
              self.Tabby();
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.fullForwardId)
            {
              self.detailx = self.game.Data.StringListObj[self.strId].Length;
              self.Tabby();
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.fullBackwardId)
            {
              self.detailx = 0;
              if (self.detailx > self.game.Data.StringListObj[self.strId].Length)
                self.detailx = self.game.Data.StringListObj[self.strId].Length;
              self.Tabby();
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.switchId)
            {
              self.mode = self.mode != 1 ? 1 : 2;
              self.Tabby();
              self.DoStuff();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.tableId)
            {
              Coordinate coordinate = self.SubPartList[index1].Click2(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (coordinate.x > -1)
              {
                if (self.mode == 1)
                {
                  self.detailx = coordinate.x;
                  self.detaily = coordinate.y;
                  if (self.detaily > self.game.Data.StringListObj[self.strId].Width)
                    self.detaily = self.game.Data.StringListObj[self.strId].Width;
                  if (self.detailx > self.game.Data.StringListObj[self.strId].Length)
                    self.detailx = self.game.Data.StringListObj[self.strId].Length;
                }
                else
                {
                  self.detaily = coordinate.x;
                  if (self.detaily > self.game.Data.StringListObj[self.strId].Width)
                    self.detaily = self.game.Data.StringListObj[self.strId].Width;
                }
                self.RefreshCellInfo();
                self.Tabby();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.text1id)
            {
              self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.listId)
            {
              let mut num4: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num4 > -1)
              {
                self.strId = num4;
                self.detailx = -1;
                self.detaily = -1;
                self.DoStuff();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.libTableId)
            {
              let mut num5: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num5 > -1)
              {
                self.libSelect = num5;
                self.strId = -1;
                self.detailx = -1;
                self.detaily = -1;
                self.DoStuff();
              }
              else if (num5 == -2)
              {
                self.libSelect = -1;
                self.detailx = -1;
                self.detaily = -1;
                self.DoStuff();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.importCsv)
            {
              str9: String = self.game.HandyFunctionsObj.LoadSomething("CSV (*.csv)|*.csv|TXT (*.txt)|*.txt|All (*.*)|*.*", "Select file to import...", self.game.AppPath + "csv/", false);
              while (self.game.Data.StringListObj[self.strId].Length > -1)
                self.game.Data.StringListObj[self.strId].RemoveRow(self.game.Data.StringListObj[self.strId].Length);
              if (Strings.Len(str9) < 2)
              {
                let mut num6: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                windowReturnClass2: WindowReturnClass;
                try
                {
                  StreamReader streamReader = File.OpenText(str9);
                  let mut num7: i32 = 0;
                  str10: String = ",";
                  let mut index10: i32 = 0;
                  try
                  {
                    bool flag1 = false;
                    if (self.game.Data.Product >= 7 && Interaction.MsgBox( "Inverse import?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    {
                      flag1 = true;
                      num7 = 2;
                    }
                    while (!streamReader.EndOfStream)
                    {
                      str11: String = streamReader.ReadLine();
                      if ((uint) Strings.InStr(str11, "sep=") > 0U)
                      {
                        str10 = Strings.Mid(str11, 5, 1);
                        num7 = 1;
                      }
                      else
                      {
                        switch (num7)
                        {
                          case 0:
                            if (Strings.InStr(str11, "\t") > 0)
                              str10 = "\t";
                            else if (Strings.InStr(str11, ",") > 0)
                              str10 = ",";
                            else if (Strings.InStr(str11, ";") > 0)
                              str10 = ";";
                            num7 = 2;
                            continue;
                          case 1:
                            num7 = 2;
                            continue;
                          case 2:
                            bool flag2 = true;
                            while (flag2)
                            {
                              flag2 = false;
                              let mut num8: i32 = Strings.InStr(str11, "\"");
                              if (num8 > 0)
                              {
                                let mut num9: i32 = Strings.InStr(num8 + 1, str11, "\"");
                                if (num9 > 0)
                                {
                                  str12: String = Strings.Left(str11, num8 - 1);
                                  str13: String = Strings.Mid(str11, num8 + 1, num9 - num8 - 1);
                                  str14: String = Strings.Mid(str11, num9 + 1);
                                  str15: String = str13.Replace(",", "¢");
                                  str11 = str12 + str15 + str14;
                                  flag2 = true;
                                }
                              }
                            }
                            strArray: Vec<String> = str11.Split(Conversions.ToChar(str10));
                            if (!flag1 && strArray.GetUpperBound(0) > self.game.Data.StringListObj[self.strId].Width & index10 == 0 && Interaction.MsgBox( "Allow adding columns to table?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                            {
                              let mut num10: i32 = strArray.GetUpperBound(0) - self.game.Data.StringListObj[self.strId].Width;
                              for (let mut index11: i32 = 1; index11 <= num10; index11 += 1)
                                self.game.Data.StringListObj[self.strId].AddCol(self.game.Data.StringListObj[self.strId].Width);
                            }
                            if (!flag1)
                            {
                              self.game.Data.StringListObj[self.strId].AddRow(index10 - 1);
                              let mut upperBound: i32 = strArray.GetUpperBound(0);
                              for (let mut index12: i32 = 0; index12 <= upperBound; index12 += 1)
                              {
                                if (index12 <= self.game.Data.StringListObj[self.strId].Width)
                                {
                                  str16: String = strArray[index12].Replace("¢", ",");
                                  self.game.Data.StringListObj[self.strId].Data[index10, index12] = str16;
                                }
                              }
                            }
                            else
                            {
                              if (index10 > self.game.Data.StringListObj[self.strId].Width)
                                self.game.Data.StringListObj[self.strId].AddCol(self.game.Data.StringListObj[self.strId].Width);
                              let mut upperBound: i32 = strArray.GetUpperBound(0);
                              for (let mut index13: i32 = 0; index13 <= upperBound; index13 += 1)
                              {
                                if (index13 - 1 > self.game.Data.StringListObj[self.strId].Length)
                                  self.game.Data.StringListObj[self.strId].AddRow(self.game.Data.StringListObj[self.strId].Length);
                                str17: String = strArray[index13].Replace("¢", ",");
                                if (index13 == 0)
                                  self.game.Data.StringListObj[self.strId].ColumnName[index10] = str17;
                                else
                                  self.game.Data.StringListObj[self.strId].Data[index13 - 1, index10] = str17;
                              }
                            }
                            index10 += 1;
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
                    streamReader.Close();
                    self.game.Data.LoadGraphics(self.formref);
                    let mut num11: i32 =  Interaction.MsgBox( ("Import problem encountered: '" + exception.Message + "'. Your current data might be corrupted. "), Title: ( "Shadow Empire : Planetary Conquest"));
                    self.RemoveSubPart(self.tableId);
                    self.tableId = 0;
                    self.DoStuff();
                    windowReturnClass1.SetFlag(true);
                    windowReturnClass2 = windowReturnClass1;
                    ProjectData.ClearProjectError();
                    goto label_216;
                  }
                  streamReader.Close();
                  self.game.Data.LoadGraphics(self.formref);
                  let mut num12: i32 =  Interaction.MsgBox( "Import finished", Title: ( "Shadow Empire : Planetary Conquest"));
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  windowReturnClass2 = windowReturnClass1;
                }
                catch (Exception ex)
                {
                  ProjectData.SetProjectError(ex);
                  let mut num13: i32 =  Interaction.MsgBox( "Error opening file", Title: ( "Shadow Empire : Planetary Conquest"));
                  ProjectData.ClearProjectError();
                  continue;
                }
label_216:
                return windowReturnClass2;
              }
            }
            else if (num1 == self.exportCsv)
            {
              str18: String = self.game.HandyFunctionsObj.SaveSomething("CSV (*.csv)|*.csv|TXT (*.txt)|*.txt|All (*.*)|*.*", "Give save name...", self.game.AppPath + "csv/", false);
              if (Strings.Len(str18) < 2)
              {
                let mut num14: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                bool flag3 = false;
                bool flag4 = false;
                if (self.game.Data.Product >= 7)
                {
                  if (Interaction.MsgBox( "Inverse export?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    flag3 = true;
                }
                try
                {
                  StreamWriter text = File.CreateText(str18);
                  try
                  {
                    if (!flag3)
                    {
                      str19: String = "";
                      let mut width3: i32 = self.game.Data.StringListObj[self.strId].Width;
                      for (let mut index14: i32 = 0; index14 <= width3; index14 += 1)
                      {
                        if (index14 > 0)
                          str19 += "\t";
                        str19 += self.game.Data.StringListObj[self.strId].ColumnName[index14];
                      }
                      text.WriteLine(str19);
                      let mut length: i32 = self.game.Data.StringListObj[self.strId].Length;
                      for (let mut row: i32 = 0; row <= length; row += 1)
                      {
                        str20: String = "";
                        let mut width4: i32 = self.game.Data.StringListObj[self.strId].Width;
                        for (let mut col: i32 = 0; col <= width4; col += 1)
                        {
                          if (col > 0)
                            str20 += "\t";
                          valueText: String = self.game.Data.StringListObj[self.strId].Data[row, col];
                          if (flag4)
                            valueText = self.game.Data.StringListObj[self.strId].GetValue( self.game.Data, row, col, self.game.Data.StringListObj[self.strId].LibId.libSlot).valueText;
                          str20 += valueText;
                        }
                        text.WriteLine(str20);
                      }
                    }
                    else
                    {
                      let mut width: i32 = self.game.Data.StringListObj[self.strId].Width;
                      for (let mut index15: i32 = 0; index15 <= width; index15 += 1)
                      {
                        str21: String = self.game.Data.StringListObj[self.strId].ColumnName[index15];
                        let mut length: i32 = self.game.Data.StringListObj[self.strId].Length;
                        for (let mut index16: i32 = 0; index16 <= length; index16 += 1)
                          str21 = str21 + "\t" + self.game.Data.StringListObj[self.strId].Data[index16, index15];
                        text.WriteLine(str21);
                      }
                    }
                    text.Close();
                    let mut num15: i32 =  Interaction.MsgBox( "Export has been written to the csv/ directory", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    Exception exception = ex;
                    text.Close();
                    let mut num16: i32 =  Interaction.MsgBox( ("Problem writing: " + exception.Message), Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.ClearProjectError();
                  }
                }
                catch (Exception ex)
                {
                  ProjectData.SetProjectError(ex);
                  let mut num17: i32 =  Interaction.MsgBox( "Problem writing. Check if the file is not opened in other application please.", Title: ( "Shadow Empire : Planetary Conquest"));
                  ProjectData.ClearProjectError();
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
  }
}
