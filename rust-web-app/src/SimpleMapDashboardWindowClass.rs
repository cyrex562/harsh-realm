// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleMapDashboardWindowClass
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
  pub class SimpleMapDashboardWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     loadMapId: i32;
     setdateid: i32;
     exportid: i32;
     setdescriptid: i32;
     setnameid: i32;
     setdesignid: i32;
     loadMapIdB: i32;
     saveId: i32;
     newId: i32;
     textId: i32;
     text2id: i32;
     text3id: i32;
     detailnr: i32;
     currentStep: i32;
     loadLayer: i32;
     removeLayer: i32;
     removeLayerB: i32;
     rleft: i32;
     rtop: i32;
     rbottom: i32;
     rright: i32;
     aleft: i32;
     atop: i32;
     abottom: i32;
     aright: i32;
     e1id: i32;
     e2id: i32;
     e3id: i32;

    pub SimpleMapDashboardWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 50, 9, tDoBorders: 1, tHeaderString: "Map Options")
    {
      self.detailnr = -1;
      self.DoStuff();
    }

    pub fn PopUpRefresh() => self.DoStuff();

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      if (self.loadMapId > 0)
        self.RemoveSubPart(self.loadMapId);
      if (self.saveId > 0)
        self.RemoveSubPart(self.saveId);
      if (self.newId > 0)
        self.RemoveSubPart(self.newId);
      if (self.textId > 0)
        self.RemoveSubPart(self.textId);
      if (self.text2id > 0)
        self.RemoveSubPart(self.text2id);
      if (self.text3id > 0)
        self.RemoveSubPart(self.text3id);
      if (self.rleft > 0)
        self.RemoveSubPart(self.rleft);
      if (self.rright > 0)
        self.RemoveSubPart(self.rright);
      if (self.rbottom > 0)
        self.RemoveSubPart(self.rbottom);
      if (self.rtop > 0)
        self.RemoveSubPart(self.rtop);
      if (self.aleft > 0)
        self.RemoveSubPart(self.aleft);
      if (self.aright > 0)
        self.RemoveSubPart(self.aright);
      if (self.abottom > 0)
        self.RemoveSubPart(self.abottom);
      if (self.atop > 0)
        self.RemoveSubPart(self.atop);
      if (self.loadLayer > 0)
        self.RemoveSubPart(self.loadLayer);
      if (self.removeLayer > 0)
        self.RemoveSubPart(self.removeLayer);
      if (self.removeLayerB > 0)
        self.RemoveSubPart(self.removeLayer);
      if (self.e1id > 0)
        self.RemoveSubPart(self.e1id);
      if (self.e2id > 0)
        self.RemoveSubPart(self.e2id);
      if (self.e3id > 0)
        self.RemoveSubPart(self.e3id);
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objgraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objgraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objgraphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 50, -1);
      let mut y1: i32 = 60;
      tText1: String = "We are using the ruleset : '" + self.game.Data.RuleSetName + "'. This cannot be changed.";
      DrawMod.DrawTextColouredMarc( objgraphics, "Ruleset", self.game.MarcFont1, num1 + 25, y1, Color.White);
      let mut num2: i32 = y1 + 0;
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText1, 27,  self.OwnBitmap, num1 + 10, num2, true, true);
      self.textId = self.AddSubPart( tsubpart1, num1 + 10, num2, 450, 108, 0);
      let mut y2: i32 = num2 + 80;
      tText2: String = "Click to load or save a map file. Keep in mind you can only load a map using the ruleset";
      DrawMod.DrawTextColouredMarc( objgraphics, "File Ops", self.game.MarcFont1, num1 + 25, y2, Color.White);
      let mut num3: i32 = y2 + 0;
      let mut tsubpart2: SubPartClass =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText2, 27,  self.OwnBitmap, num1 + 10, num3, true, true);
      self.text2id = self.AddSubPart( tsubpart2, num1 + 10, num3, 450, 108, 0);
      let mut num4: i32 = num3 + 70;
      let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Load Map", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadMapId = self.AddSubPart( tsubpart3, num1 + 25, num4, 140, 35, 1);
      let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Save Map", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 160), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.saveId = self.AddSubPart( tsubpart4, num1 + 25 + 160, num4, 140, 35, 1);
      let mut tsubpart5: SubPartClass =  new TextButtonPartClass("New Map", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 320), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.newId = self.AddSubPart( tsubpart5, num1 + 25 + 320, num4, 140, 35, 1);
      let mut y3: i32 = num4 + 50;
      tText3: String = "These options allow you to crop or enlarge the current map.";
      DrawMod.DrawTextColouredMarc( objgraphics, "Map Dimensions", self.game.MarcFont1, num1 + 25, y3, Color.White);
      let mut num5: i32 = y3 + 0;
      let mut tsubpart6: SubPartClass =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText3, 27,  self.OwnBitmap, num1 + 10, num5, true, true);
      self.text2id = self.AddSubPart( tsubpart6, num1 + 10, num5, 450, 108, 0);
      let mut num6: i32 = num5 + 70;
      let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Remove Left", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25), bby: num6, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.rleft = self.AddSubPart( tsubpart7, num1 + 25, num6, 140, 35, 1);
      let mut tsubpart8: SubPartClass =  new TextButtonPartClass("Remove Top", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 160), bby: num6, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.rtop = self.AddSubPart( tsubpart8, num1 + 25 + 160, num6, 140, 35, 1);
      let mut tsubpart9: SubPartClass =  new TextButtonPartClass("Remove Right", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 320), bby: num6, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.rright = self.AddSubPart( tsubpart9, num1 + 25 + 320, num6, 140, 35, 1);
      let mut tsubpart10: SubPartClass =  new TextButtonPartClass("Remove Bottom", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 480), bby: num6, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.rbottom = self.AddSubPart( tsubpart10, num1 + 25 + 480, num6, 140, 35, 1);
      let mut num7: i32 = num6 + 45;
      let mut tsubpart11: SubPartClass =  new TextButtonPartClass("Add Left", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25), bby: num7, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.aleft = self.AddSubPart( tsubpart11, num1 + 25, num7, 140, 35, 1);
      let mut tsubpart12: SubPartClass =  new TextButtonPartClass("Add Top", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 160), bby: num7, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.atop = self.AddSubPart( tsubpart12, num1 + 25 + 160, num7, 140, 35, 1);
      let mut tsubpart13: SubPartClass =  new TextButtonPartClass("Add Right", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 320), bby: num7, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.aright = self.AddSubPart( tsubpart13, num1 + 25 + 320, num7, 140, 35, 1);
      let mut tsubpart14: SubPartClass =  new TextButtonPartClass("Add Bottom", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 480), bby: num7, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.abottom = self.AddSubPart( tsubpart14, num1 + 25 + 480, num7, 140, 35, 1);
      let mut y4: i32 = num7 + 50;
      tText4: String = "You can load an image file to be transparently overlaid the map to help you trace coastlines, position cities, etc...";
      DrawMod.DrawTextColouredMarc( objgraphics, "Transparent overlay aid", self.game.MarcFont1, num1 + 25, y4, Color.White);
      let mut num8: i32 = y4 + 0;
      let mut tsubpart15: SubPartClass =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText4, 27,  self.OwnBitmap, num1 + 10, num8, true, true);
      self.text3id = self.AddSubPart( tsubpart15, num1 + 10, num8, 450, 108, 0);
      let mut num9: i32 = num8 + 100;
      let mut tsubpart16: SubPartClass =  new TextButtonPartClass("(Re)load overlay", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25), bby: num9, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadLayer = self.AddSubPart( tsubpart16, num1 + 25, num9, 140, 35, 1);
      if (self.game.Data.PermanentOverlayUse)
      {
        tsubpart16 =  new TextButtonPartClass("Remove overlay", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 160), bby: num9, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeLayer = self.AddSubPart( tsubpart16, num1 + 25 + 160, num9, 140, 35, 1);
      }
      else
      {
        tsubpart16 =  new TextButtonPartClass("Remove overlay", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 160), bby: num9, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeLayerB = self.AddSubPart( tsubpart16, num1 + 25 + 160, num9, 140, 35, 1);
      }
      let mut y5: i32 = num9 + 50;
      mapDesigner: String = self.game.Data.MapDesigner;
      DrawMod.DrawTextColouredMarc( objgraphics, "Designer", self.game.MarcFont1, num1 + 25, y5, Color.White);
      tsubpart16 =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, mapDesigner, 27,  self.OwnBitmap, num1 + 10, y5 - 5, true, true);
      self.text3id = self.AddSubPart( tsubpart16, num1 + 10, y5 - 5, 250, 108, 0);
      tsubpart16 =  new TextButtonPartClass("Change", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25), bby: (y5 + 70), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.e1id = self.AddSubPart( tsubpart16, num1 + 25, y5 + 70, 140, 35, 1);
      tText5: String = self.game.Data.MapVersion.ToString();
      DrawMod.DrawTextColouredMarc( objgraphics, "Map version", self.game.MarcFont1, num1 + 25 + 250, y5, Color.White);
      tsubpart16 =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, tText5, 27,  self.OwnBitmap, num1 + 10 + 250, y5 - 5, true, true);
      self.text3id = self.AddSubPart( tsubpart16, num1 + 10 + 250, y5 - 5, 250, 108, 0);
      tsubpart16 =  new TextButtonPartClass("Change", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 250), bby: (y5 + 70), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.e3id = self.AddSubPart( tsubpart16, num1 + 25 + 250, y5 + 70, 140, 35, 1);
      mapName: String = self.game.Data.MapName;
      DrawMod.DrawTextColouredMarc( objgraphics, "Map name", self.game.MarcFont1, num1 + 25 + 500, y5, Color.White);
      tsubpart16 =  new TextAreaClass2(self.game, 850, 4, self.game.MarcFont3, mapName, 27,  self.OwnBitmap, num1 + 10 + 500, y5 - 5, true, true);
      self.text3id = self.AddSubPart( tsubpart16, num1 + 10 + 500, y5 - 5, 550, 108, 0);
      tsubpart16 =  new TextButtonPartClass("Change", 140, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 25 + 500), bby: (y5 + 70), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.e2id = self.AddSubPart( tsubpart16, num1 + 25 + 500, y5 + 70, 140, 35, 1);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.e1id)
            {
              self.game.Data.MapDesigner = Interaction.InputBox("Give designer name", "Shadow Empire : Planetary Conquest", self.game.Data.MapDesigner);
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.e2id)
            {
              self.game.Data.MapName = Interaction.InputBox("Give map name", "Shadow Empire : Planetary Conquest", self.game.Data.MapName);
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.e3id)
            {
              let mut num2: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give version number", "Shadow Empire : Planetary Conquest", self.game.Data.MapVersion.ToString())));
              if (num2 > 0)
              {
                self.game.Data.MapVersion = num2;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == self.aleft)
              {
                let mut tempint: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Add howmany columns of hexes? (2-100)", "Shadow Empire : Planetary Conquest")));
                if (tempint > 0 & tempint <= 100)
                {
                  if ((tempint + 2) % 2 > 0)
                  {
                    let mut num3: i32 =  Interaction.MsgBox( "You can only add an EVEN number of columns", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    self.game.HandyFunctionsObj.AddXToMapLeft(tempint);
                    let mut num4: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.aright)
              {
                let mut tempint: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Add howmany columns of hexes? (1-100)", "Shadow Empire : Planetary Conquest")));
                if (tempint > 0 & tempint <= 100)
                {
                  self.game.HandyFunctionsObj.AddXToMap(tempint);
                  let mut num5: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.atop)
              {
                let mut tempint: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Add howmany rows of hexes? (1-100)", "Shadow Empire : Planetary Conquest")));
                if (tempint > 0 & tempint <= 100)
                {
                  self.game.HandyFunctionsObj.AddYToMapLeft(tempint);
                  let mut num6: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.abottom)
              {
                let mut tempint: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Add howmany rows of hexes? (1-100)", "Shadow Empire : Planetary Conquest")));
                if (tempint > 0 & tempint <= 100)
                {
                  self.game.HandyFunctionsObj.AddYToMap(tempint);
                  let mut num7: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.rleft)
              {
                let mut num8: i32 = self.game.Data.MapObj[0].MapWidth + 1;
                if (num8 < 2)
                {
                  let mut num9: i32 =  Interaction.MsgBox( "Not enough hexes for operation.", Title: ( "Shadow Empire : Planetary Conquest"));
                  return windowReturnClass;
                }
                let mut tempint: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Remove howmany columns of hexes? (2-" + ( Math.Round(Math.Ceiling( num8 / 2.0) * 2.0)).ToString() + ")", "Shadow Empire : Planetary Conquest")));
                if (tempint > 0 & tempint <= num8)
                {
                  if ((tempint + 2) % 2 > 0)
                  {
                    let mut num10: i32 =  Interaction.MsgBox( "You can only add an EVEN number of columns", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    self.game.HandyFunctionsObj.RemoveXToMapLeft(tempint);
                    let mut num11: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.rright)
              {
                let mut num12: i32 = self.game.Data.MapObj[0].MapWidth + 1;
                if (num12 < 2)
                {
                  let mut num13: i32 =  Interaction.MsgBox( "Not enough hexes for operation.", Title: ( "Shadow Empire : Planetary Conquest"));
                  return windowReturnClass;
                }
                let mut tempint: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Remove howmany columns of hexes? (1-" + num12.ToString() + ")", "Shadow Empire : Planetary Conquest")));
                if (tempint > 0 & tempint <= num12)
                {
                  self.game.HandyFunctionsObj.RemoveXToMap(tempint);
                  let mut num14: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.rtop)
              {
                let mut num15: i32 = self.game.Data.MapObj[0].MapHeight + 1;
                if (num15 < 2)
                {
                  let mut num16: i32 =  Interaction.MsgBox( "Not enough hexes for operation.", Title: ( "Shadow Empire : Planetary Conquest"));
                  return windowReturnClass;
                }
                let mut tempint: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Remove howmany rows of hexes? (1-" + num15.ToString() + ")", "Shadow Empire : Planetary Conquest")));
                if (tempint > 0 & tempint <= 100)
                {
                  self.game.HandyFunctionsObj.RemoveYToMapLeft(tempint);
                  let mut num17: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.rbottom)
              {
                let mut num18: i32 = self.game.Data.MapObj[0].MapHeight + 1;
                if (num18 < 2)
                {
                  let mut num19: i32 =  Interaction.MsgBox( "Not enough hexes for operation.", Title: ( "Shadow Empire : Planetary Conquest"));
                  return windowReturnClass;
                }
                let mut tempint: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Remove howmany rows of hexes? (1-" + num18.ToString() + ")", "Shadow Empire : Planetary Conquest")));
                if (tempint > 0 & tempint <= 100)
                {
                  self.game.HandyFunctionsObj.RemoveYToMap(tempint);
                  let mut num20: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.loadLayer)
              {
                str: String = self.game.HandyFunctionsObj.LoadSomething("JPG(*.jpg)|*.jpg|PNG(*.png)|*.png", "Pick a graphic... press cancel to delete the overlay graphic. graphic selected must be in graphics directory", self.game.AppPath + "graphics/", true);
                if (File.Exists(self.game.AppPath + "graphics/" + str))
                {
                  self.game.Data.PermanentOverlayName = str;
                  self.game.Data.PermanentOverlayUse = true;
                  self.game.Data.LoadSprites();
                }
                else
                {
                  let mut num21: i32 =  Interaction.MsgBox( "Could not find graphic. Make sure its located inside the ''graphics'' directory", Title: ( "Shadow Empire : Planetary Conquest"));
                  self.game.Data.PermanentOverlayUse = false;
                  self.game.Data.PermanentOverlayName = "systemgraphics/trans.bmp";
                  self.game.Data.LoadSprites();
                }
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.removeLayer)
              {
                self.game.Data.PermanentOverlayUse = false;
                self.game.Data.PermanentOverlayName = "systemgraphics/trans.bmp";
                self.game.Data.LoadSprites();
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.newId)
              {
                str1: String = Interaction.InputBox("Give width of map in hexes (10-200)", "Shadow Empire : Planetary Conquest");
                let mut twidth: i32 = Operators.CompareString(Strings.Trim(str1), "", false) == 0 ? 0 : Conversions.ToInteger(str1);
                str2: String = Interaction.InputBox("Give height of map in hexes (10-200)", "Shadow Empire : Planetary Conquest");
                let mut theight: i32 = Operators.CompareString(Strings.Trim(str2), "", false) == 0 ? 0 : Conversions.ToInteger(str2);
                if (twidth < 10 | theight < 10 | twidth > 200 | theight > 200)
                {
                  let mut num22: i32 =  Interaction.MsgBox( "Cannot comply. Width and Height must be between 10 and 200", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  self.game.FormRef.Cursor = Cursors.WaitCursor;
                  self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                  filename: String = self.game.AppPath + self.game.ModScenarioDir + "/" + self.game.Data.MasterFile;
                  self.game.Data = new DataClass(twidth, theight);
                  self.game.HandyFunctionsObj.LoadMasterFile(filename);
                  BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                  self.game.Data.LoadGraphics((Form1) null);
                  self.game.Data.SimpleEditor = true;
                  self.game.EditObj.inSimpleMapEditor = true;
                  self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                  self.game.FormRef.Cursor = Cursors.Default;
                  self.game.SelectX = 0;
                  self.game.SelectY = 0;
                  self.game.CornerX = 0;
                  self.game.CornerY = 0;
                  let mut num23: i32 =  Interaction.MsgBox( "Done", Title: ( "Shadow Empire : Planetary Conquest"));
                  self.game.EditObj.MiddleWindow = true;
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
              else
              {
                if (num1 == self.loadMapId)
                {
                  path: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Map file(*.se1map)|*.se1map", "Pick a map to load...", self.game.AppPath + self.game.ModScenarioDir, false);
                  if (File.Exists(path))
                  {
                    self.game.EditObj.TempFileName = path;
                    self.game.EditObj.PopupValue = 18;
                    self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                    windowReturnClass.AddCommand(5, 10);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  let mut num24: i32 =  Interaction.MsgBox( "Could not find file", Title: ( "Shadow Empire : Planetary Conquest"));
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == self.saveId)
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
                  str: String = self.game.HandyFunctionsObj.SaveSomething("SE1 Map file(*.se1map)|*.se1map", "Give save name...", tinitdir, false);
                  if (Strings.Len(str) < 2)
                  {
                    let mut num25: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.Data.serialize(str);
                    self.game.HandyFunctionsObj.ZipFile(str);
                    windowReturnClass.SetFlag(true);
                    self.game.FormRef.Cursor = Cursors.Default;
                    self.game.Data.LoadGraphics(self.formref);
                    self.game.Data.PermanentOverlaySpriteID = -1;
                    self.game.Data.PermanentOverlayUse = false;
                    let mut num26: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
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
