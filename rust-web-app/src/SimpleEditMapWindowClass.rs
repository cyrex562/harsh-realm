// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleEditMapWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;

namespace WindowsApplication1
{
  pub class SimpleEditMapWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     loadMapId: i32;
     loadMapIdB: i32;
     textId: i32;
     detailnr: i32;
     currentStep: i32;
     pickid: i32;
     opt1id: i32;
     opt2id: i32;
     opt3id: i32;
     opt4id: i32;
     opt5id: i32;
     opt6id: i32;
     opt7id: i32;
     opt8id: i32;
     opt9id: i32;
     opt10id: i32;
     opt11id: i32;
     opt12id: i32;
     ListClass VPListOBj;
     miniId: i32;
     VPListId: i32;
     AddVPId: i32;
     AddVPIdb: i32;

    pub SimpleEditMapWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, 300, 9, tDoBorders: 1, tHeaderString: "Map")
    {
      self.detailnr = -1;
      self.DoStuff();
    }

    pub fn DoRefresh() => self.DoStuff();

    pub fn PopUpRefresh() => self.formref.Screeny.FlagAllIncludingRefresh();

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 350, -1);
      DrawMod.DrawBlock( graphics, 2, 294, DrawMod.TGame.ScreenWidth - 4, 4, 0, 0, 0, 128);
      if (self.VPListId > 0)
        self.RemoveSubPart(self.VPListId);
      if (self.AddVPId > 0)
        self.RemoveSubPart(self.AddVPId);
      if (self.AddVPIdb > 0)
        self.RemoveSubPart(self.AddVPIdb);
      self.VPListOBj = ListClass::new();
      let mut num2: i32 = -1;
      let mut num3: i32 = -1;
      let mut num4: i32 = 0;
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut x: i32 = 0; x <= mapWidth; x += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut y: i32 = 0; y <= mapHeight; y += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[x, y].VP > 0)
          {
            num3 += 1;
            num4 += self.game.Data.MapObj[0].HexObj[x, y].VP;
            self.VPListOBj.add(x.ToString() + "," + y.ToString() + " VP=" + self.game.Data.MapObj[0].HexObj[x, y].VP.ToString() + ", " + self.game.HandyFunctionsObj.GetHexName(x, y, 0), x * 10000 + y);
            if (self.game.SelectX == x & self.game.SelectY == y)
              num2 = num3;
          }
        }
      }
      self.VPListOBj.add("Total VP on map = " + num4.ToString(), -2);
      ListClass vpListObj = self.VPListOBj;
      let mut tlistselect: i32 = num2;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      let mut bbx: i32 = 10 + num1;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(vpListObj, 12, 200, tlistselect, game, true, "Victory Points (VP)", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx, bby: 52, tMarcStyle: true, overruleFont: ( local2));
      self.VPListId = self.AddSubPart( tsubpart, 10 + num1, 52, 220, 208, 0);
      let mut num5: i32 = num1 + 250;
      let mut y1: i32 = 50;
      if (self.game.SelectX > -1 & self.game.SelectY > -1)
      {
        DrawMod.DrawTextColouredMarc( graphics, "Selected hex:", self.game.MarcFont4, num5, y1, Color.White);
        tstring1: String = self.game.SelectX.ToString() + "," + self.game.SelectY.ToString() + "," + self.game.HandyFunctionsObj.GetHexName(self.game.SelectX, self.game.SelectY, 0);
        if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].HeightLevel > 0)
          tstring1 = tstring1 + " (Lvl " + self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].HeightLevel.ToString() + ")";
        DrawMod.DrawTextColouredMarc( graphics, tstring1, self.game.MarcFont3, num5, y1 + 20, Color.White);
        tstring2: String = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime <= -1 ? "no regime set" : self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime].Name;
        DrawMod.DrawTextColouredMarc( graphics, tstring2, self.game.MarcFont3, num5, y1 + 40, Color.White);
        tstring3: String = "none";
        if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location > -1)
        {
          tstring3 = self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location].Type].Name;
          if (self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location].People > -1)
            tstring3 = tstring3 + ", " + self.game.Data.PeopleObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location].People].Name;
        }
        DrawMod.DrawTextColouredMarc( graphics, "Location Type + People:", self.game.MarcFont4, num5, y1 + 70, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, tstring3, self.game.MarcFont3, num5, y1 + 90, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, "Victory Points (VP):", self.game.MarcFont4, num5, y1 + 120, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].VP.ToString(), self.game.MarcFont3, num5, y1 + 140, Color.White);
        tsubpart =  new TextButtonPartClass("Change VP of hex", 200, "Click to change the VP on this hex.",  self.OwnBitmap, num5, y1 + 180, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.AddVPId = self.AddSubPart( tsubpart, num5, y1 + 180, 200, 35, 1);
      }
      else
      {
        DrawMod.DrawTextColouredMarc( graphics, "Selected hex:", self.game.MarcFont4, num5, y1, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, "None", self.game.MarcFont3, num5, y1 + 20, Color.White);
        tsubpart =  new TextButtonPartClass("Change VP of hex", 200, "You have to select a hex on the map first.",  self.OwnBitmap, num5, y1 + 120, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.AddVPIdb = self.AddSubPart( tsubpart, num5, y1 + 120, 200, 35, 1);
      }
      if (self.pickid > 0)
        self.RemoveSubPart(self.pickid);
      if (self.opt1id > 0)
        self.RemoveSubPart(self.opt1id);
      if (self.opt2id > 0)
        self.RemoveSubPart(self.opt2id);
      if (self.opt3id > 0)
        self.RemoveSubPart(self.opt3id);
      if (self.opt4id > 0)
        self.RemoveSubPart(self.opt4id);
      if (self.opt5id > 0)
        self.RemoveSubPart(self.opt5id);
      if (self.opt6id > 0)
        self.RemoveSubPart(self.opt6id);
      if (self.opt7id > 0)
        self.RemoveSubPart(self.opt7id);
      if (self.opt8id > 0)
        self.RemoveSubPart(self.opt8id);
      if (self.opt9id > 0)
        self.RemoveSubPart(self.opt9id);
      if (self.opt10id > 0)
        self.RemoveSubPart(self.opt10id);
      if (self.opt12id > 0)
        self.RemoveSubPart(self.opt12id);
      let mut num6: i32 = num1 + 500;
      let mut y2: i32 = 50;
      bool flag1;
      str1: String;
      str2: String;
      if (self.game.EditObj.PencilType > 0)
      {
        if (!(self.game.EditObj.PencilType == 3 | self.game.EditObj.PencilType == 10 | self.game.EditObj.PencilType == 12 | self.game.EditObj.PencilType == 11 | self.game.EditObj.PencilType == 1 | self.game.EditObj.PencilType == 9))
          flag1 = true;
        str3: String;
        if (self.game.EditObj.PencilType == 1 | self.game.EditObj.PencilType == 10)
        {
          str1 = "Landsc# " + Conversion.Str( self.game.EditObj.PencilData1) + "," + Conversion.Str( self.game.EditObj.PencilData2);
          str2 = self.game.Data.LandscapeTypeObj[self.game.EditObj.PencilData1].Name;
          str3 = "Left click to draw this landscape+sprite on a hex, right click to only select a hex.";
        }
        else if (self.game.EditObj.PencilType == 2)
        {
          str1 = "Road# " + Conversion.Str( self.game.EditObj.PencilData1);
          str2 = self.game.Data.RoadTypeObj[self.game.EditObj.PencilData1].Name;
          str3 = "First right click to select a hex, then left click on a neighbouring hex to draw a road between them.";
        }
        else if (self.game.EditObj.PencilType == 3)
        {
          str1 = "Reg# " + Conversion.Str( self.game.EditObj.PencilData1);
          str2 = self.game.EditObj.PencilData1 <= self.game.Data.RegimeCounter ? self.game.Data.RegimeObj[self.game.EditObj.PencilData1].Name : "Neutral Hex";
          str3 = "Left click to draw this regime on a hex, right click just to select a hex, clicking twice results in hex becoming neutral again.";
        }
        else if (self.game.EditObj.PencilType == 4)
        {
          str1 = "LocTyp# " + Conversion.Str( self.game.EditObj.PencilData1);
          str2 = self.game.Data.LocTypeObj[self.game.EditObj.PencilData1].Name;
          str3 = "Left click on a hex to place a location of this locationtype. Right click is only select.";
        }
        else if (self.game.EditObj.PencilType == 5)
        {
          str1 = "RiverTyp# " + Conversion.Str( self.game.EditObj.PencilData1);
          str2 = self.game.Data.RiverTypeObj[self.game.EditObj.PencilData1].Name;
          str3 = "First right click to select a hex, then left click on a neighbouring hex to draw a river inbetween them.";
        }
        else if (self.game.EditObj.PencilType == 6)
        {
          str1 = "Bridge";
          str2 = "";
          str3 = "First right click to select a hex, then left click on a neighbouring hex to draw a bridge in between them.";
        }
        else if (self.game.EditObj.PencilType == 9)
        {
          str1 = "Slot# " + Conversion.Str( self.game.EditObj.PencilData1) + ", => " + Conversion.Str( self.game.EditObj.PencilData2);
          str2 = "";
          str3 = "Left click to place this value on a hex, right click just to select a hex.";
        }
        else if (self.game.EditObj.PencilType == 11)
        {
          str1 = "Hex LibVar# " + Conversion.Str( self.game.EditObj.PencilData1);
          str2 = "value " + Conversion.Str( self.game.EditObj.PencilData2);
          str3 = "Left click to place this value on a hex, right click just to select a hex.";
        }
        else if (self.game.EditObj.PencilType == 12)
        {
          str1 = "Height Level";
          str2 = Conversion.Str( self.game.EditObj.PencilData2);
          str3 = "Left click to place this value on a hex.";
        }
        else
        {
          str1 = "Pointer";
          str2 = "";
          str3 = "I hope you are having a good day!";
        }
      }
      str4: String = self.game.EditObj.PencilMode != 0 ? "Fill" : "Draw";
      if (self.game.EditObj.PencilType == 0)
      {
        str4 = "None";
        str1 = "N/a";
      }
      DrawMod.DrawTextColouredMarc( graphics, "Draw mode:", self.game.MarcFont4, num6, y2, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, "Draw type:", self.game.MarcFont4, num6, y2 + 50, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, str1 + " " + str2, self.game.MarcFont3, num6, y2 + 18, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, str4, self.game.MarcFont3, num6, y2 + 68, Color.White);
      tsubpart =  new TextButtonPartClass("Pick Draw Type", 160, tBackbitmap: ( self.OwnBitmap), bbx: num6, bby: (y2 + 140), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.pickid = self.AddSubPart( tsubpart, num6, y2 + 140, 160, 35, 1);
      if (Operators.CompareString(str4, "Draw", false) == 0)
      {
        if (!flag1)
        {
          tsubpart =  new TextButtonPartClass("Go to fill mode", 160, tBackbitmap: ( self.OwnBitmap), bbx: num6, bby: (y2 + 100), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.opt2id = self.AddSubPart( tsubpart, num6, y2 + 100, 160, 35, 1);
        }
        else
        {
          tsubpart =  new TextButtonPartClass("Go to fill mode", 160, tBackbitmap: ( self.OwnBitmap), bbx: num6, bby: (y2 + 100), tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.opt4id = self.AddSubPart( tsubpart, num6, y2 + 100, 160, 35, 0);
        }
      }
      else if (Operators.CompareString(str4, "Fill", false) == 0)
      {
        tsubpart =  new TextButtonPartClass("Go to draw mode", 160, tBackbitmap: ( self.OwnBitmap), bbx: num6, bby: (y2 + 100), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.opt1id = self.AddSubPart( tsubpart, num6, y2 + 100, 160, 35, 1);
      }
      if (Operators.CompareString(str4, "None", false) != 0)
      {
        tsubpart =  new TextButtonPartClass("End drawing mode", 160, tBackbitmap: ( self.OwnBitmap), bbx: num6, bby: (y2 + 180), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.opt3id = self.AddSubPart( tsubpart, num6, y2 + 180, 160, 35, 1);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("Go to fill mode", 160, tBackbitmap: ( self.OwnBitmap), bbx: num6, bby: (y2 + 100), tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.opt4id = self.AddSubPart( tsubpart, num6, y2 + 100, 160, 35, 0);
        tsubpart =  new TextButtonPartClass("End drawing mode", 160, tBackbitmap: ( self.OwnBitmap), bbx: num6, bby: (y2 + 140), tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.opt5id = self.AddSubPart( tsubpart, num6, y2 + 180, 160, 35, 1);
      }
      if (self.game.EditObj.inSimpleMapEditor)
      {
        bool flag2 = true;
        if (self.game.SelectX > -1 && self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Location > -1)
          flag2 = false;
        if (!flag2)
        {
          tsubpart =  new TextButtonPartClass("Rename location", 160, tBackbitmap: ( self.OwnBitmap), bbx: (num6 + 180), bby: (y2 + 60), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.opt12id = self.AddSubPart( tsubpart, num6 + 180, y2 + 60, 160, 35, 1);
          tsubpart =  new TextButtonPartClass("Delete location", 160, tBackbitmap: ( self.OwnBitmap), bbx: (num6 + 180), bby: (y2 + 100), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.opt7id = self.AddSubPart( tsubpart, num6 + 180, y2 + 100, 160, 35, 1);
        }
        else
        {
          tsubpart =  new TextButtonPartClass("Set hex name", 160, tBackbitmap: ( self.OwnBitmap), bbx: (num6 + 180), bby: (y2 + 100), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.opt8id = self.AddSubPart( tsubpart, num6 + 180, y2 + 100, 160, 35, 1);
        }
        tsubpart =  new TextButtonPartClass("Set Labels", 160, tBackbitmap: ( self.OwnBitmap), bbx: (num6 + 180), bby: (y2 + 140), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.opt9id = self.AddSubPart( tsubpart, num6 + 180, y2 + 140, 160, 35, 1);
        tsubpart =  new TextButtonPartClass("Remove Labels", 160, tBackbitmap: ( self.OwnBitmap), bbx: (num6 + 180), bby: (y2 + 180), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.opt10id = self.AddSubPart( tsubpart, num6 + 180, y2 + 180, 160, 35, 1);
      }
      if (self.miniId > 0)
      {
        self.RemoveSubPart(self.miniId);
        self.miniId = 0;
      }
      if (self.miniId >= 1)
        return;
      tsubpart =  new MiniMapPartClass(DrawMod.TGame, tx: (100 + num1), ty: 220);
      self.miniId = self.AddSubPart( tsubpart, num1 + 860, 50, 180 + num1, 220, 0);
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
            if (num1 == self.VPListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                let mut tx: i32 =  Math.Round(Conversion.Int( num2 / 10000.0));
                let mut ty: i32 = num2 % 10000;
                self.game.SelectX = tx;
                self.game.SelectY = ty;
                self.game.HandyFunctionsObj.CenterOnXY(tx, ty, true);
                windowReturnClass.AddCommand(4, 12);
                self.DoStuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.AddVPId)
            {
              DefaultResponse: String = "";
              if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].VP > 0)
                DefaultResponse = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].VP.ToString().ToString();
              self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].VP =  Math.Round(Conversion.Val(Interaction.InputBox("Give new number value for variable", "Shadow Empire : Planetary Conquest", DefaultResponse)));
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.pickid)
            {
              self.game.EditObj.PopupValue = 16;
              self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
              windowReturnClass.AddCommand(5, 10);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.opt1id)
            {
              self.game.EditObj.PencilMode = 0;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
            }
            else
            {
              if (num1 == self.opt2id)
              {
                self.game.EditObj.PencilMode = 1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt3id)
              {
                self.game.EditObj.PencilType = 0;
                self.game.EditObj.PencilMode = 0;
                self.game.EditObj.PaintShortcut1 = -1;
                self.game.EditObj.PaintShortcut2 = -1;
                self.game.EditObj.PaintShortcut3 = -1;
                self.DoStuff();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt7id)
              {
                self.game.Data.RemoveLoc(self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Location);
                self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Location = -1;
                self.DoStuff();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt12id)
              {
                self.game.Data.LocObj[self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Location].Name = Interaction.InputBox("Give new name for location", "Shadow Empire : Planetary Conquest", self.game.Data.LocObj[self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.SelectX, self.game.SelectY].Location].Name);
                self.DoStuff();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt8id)
              {
                self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Name = Interaction.InputBox("Give new name for hex", "Shadow Empire : Planetary Conquest", self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Name);
                self.DoStuff();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt9id)
              {
                self.game.HandyFunctionsObj.MakeAutoLabels(1, 1);
                self.game.HandyFunctionsObj.MakeAutoLabels(2, 2);
                self.game.HandyFunctionsObj.MakeAutoLabels(5, 3);
                let mut num3: i32 =  Interaction.MsgBox( "Set all hex labels");
                self.DoStuff();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.opt10id)
              {
                let mut mapWidth: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth;
                for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
                {
                  let mut mapHeight: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight;
                  for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
                  {
                    self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[index2, index3].LabelText1 = "";
                    self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[index2, index3].LabelText2 = "";
                    self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[index2, index3].LabelType1 = 0;
                    self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[index2, index3].LabelType2 = 0;
                    self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[index2, index3].SmallLabel = "";
                    self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[index2, index3].SmallLabelType = 0;
                  }
                }
                let mut num4: i32 =  Interaction.MsgBox( "Removed all hex labels");
                self.DoStuff();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.miniId)
              {
                self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                windowReturnClass.AddCommand(4, 12);
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
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
