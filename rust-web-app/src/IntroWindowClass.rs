// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.IntroWindowClass
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
  pub class IntroWindowClass : WindowClass
  {
     BStartGameID: i32;
     BLoadGameID: i32;
     BSaveGameID: i32;
     BRandomID: i32;
     BEditorID: i32;
     bWebsiteID: i32;
     TempText: i32;
     TempText2: i32;
     txt1: i32;
     txt2: i32;
     txt3: i32;
     minimapid: i32;
     opt1: i32;
     opt2: i32;
     opt3: i32;
     opt4: i32;
     opt5: i32;
     opt6: i32;
     opt7: i32;
     txt7: i32;
     txt4: i32;
     txt5: i32;
     txt6: i32;
     txt8: i32;
     opt8: i32;
     txt9: i32;
     opt9: i32;
     txt10: i32;
     cancelID: i32;
     int[] vari;
     int[] varitext;
     ATListClass RegimeListObj;
     RegimeListId: i32;
     float tempBlink;
     detailnr: i32;

    pub IntroWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      this.vari = new int[13];
      this.varitext = new int[13];
      this.tempBlink = 0.0f;
      this.detailnr = -1;
      this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 395, 144, false);
      this.DoStuff();
    }

    pub fn DoStuff()
    {
      SizeF sizeF1 = SizeF::new();
      if (this.TempText > 0)
        this.RemoveSubPart(this.TempText);
      if (this.TempText2 > 0)
        this.RemoveSubPart(this.TempText2);
      if (this.BStartGameID > 0)
        this.RemoveSubPart(this.BStartGameID);
      if (this.BRandomID > 0)
        this.RemoveSubPart(this.BRandomID);
      if (this.BLoadGameID > 0)
        this.RemoveSubPart(this.BLoadGameID);
      if (this.BEditorID > 0)
        this.RemoveSubPart(this.BEditorID);
      if (this.bWebsiteID > 0)
        this.RemoveSubPart(this.bWebsiteID);
      if (this.minimapid > 0)
        this.RemoveSubPart(this.minimapid);
      if (this.opt1 > 0)
        this.RemoveSubPart(this.opt1);
      if (this.txt1 > 0)
        this.RemoveSubPart(this.txt1);
      if (this.opt2 > 0)
        this.RemoveSubPart(this.opt2);
      if (this.txt2 > 0)
        this.RemoveSubPart(this.txt2);
      if (this.opt3 > 0)
        this.RemoveSubPart(this.opt3);
      if (this.txt3 > 0)
        this.RemoveSubPart(this.txt3);
      if (this.opt4 > 0)
        this.RemoveSubPart(this.opt4);
      if (this.txt4 > 0)
        this.RemoveSubPart(this.txt4);
      if (this.opt5 > 0)
        this.RemoveSubPart(this.opt5);
      if (this.txt5 > 0)
        this.RemoveSubPart(this.txt5);
      if (this.opt6 > 0)
        this.RemoveSubPart(this.opt6);
      if (this.txt6 > 0)
        this.RemoveSubPart(this.txt6);
      if (this.opt7 > 0)
        this.RemoveSubPart(this.opt7);
      if (this.txt7 > 0)
        this.RemoveSubPart(this.txt7);
      if (this.opt8 > 0)
        this.RemoveSubPart(this.opt8);
      if (this.txt8 > 0)
        this.RemoveSubPart(this.txt8);
      if (this.opt9 > 0)
        this.RemoveSubPart(this.opt9);
      if (this.txt9 > 0)
        this.RemoveSubPart(this.txt9);
      if (this.txt10 > 0)
        this.RemoveSubPart(this.txt10);
      let mut index1: i32 =  0;
      do
      {
        if (this.vari[index1] > 0)
          this.RemoveSubPart(this.vari[index1]);
        if (this.varitext[index1] > 0)
          this.RemoveSubPart(this.varitext[index1]);
        index1 += 1;
      }
      while (index1 <= 12);
      if (this.cancelID > 0)
        this.RemoveSubPart(this.cancelID);
      this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND2MARC);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      DrawMod.DrawBlock( graphics, 35, 93, 945, 587,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  Math.Round( this.game.VicColor4.A / 2.0));
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  graphics, 35, 93, 945, 587, -1, -1);
      let mut num1: i32 =  115;
      let mut num2: i32 =  25;
      let mut index2: i32 =  0;
      SubPartClass tsubpart;
      num3: i32;
      do
      {
        if (this.game.Data.Variants[index2] > -1)
        {
          if (this.game.Data.GameSlot[this.game.Data.Variants[index2]] <= 0)
          {
            int[] vari = this.vari;
            let mut index3: i32 =  index2;
            tsubpart =  new SteveButtonPartClass25(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 276, bby: (3 + num1 + index2 * num2));
            let mut num4: i32 =  this.AddSubPart( tsubpart, 276, 3 + num1 + index2 * num2, num2, num2, 1);
            vari[index3] = num4;
          }
          else
          {
            int[] vari = this.vari;
            let mut index4: i32 =  index2;
            tsubpart =  new SteveButtonPartClass25(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 276, bby: (3 + num1 + index2 * num2));
            let mut num5: i32 =  this.AddSubPart( tsubpart, 276, 3 + num1 + index2 * num2, num2, num2, 1);
            vari[index4] = num5;
          }
          num3 += 1;
          int[] varitext = this.varitext;
          let mut index5: i32 =  index2;
          tsubpart =  new ATTextPartClass(this.game.Data.GameSlotName[this.game.Data.Variants[index2]], this.game.VicFont3, 150, 16, false, toutline: true);
          let mut num6: i32 =  this.AddSubPart( tsubpart, 305, num1 + 7 + index2 * num2, 150, 16, 0);
          varitext[index5] = num6;
        }
        index2 += 1;
      }
      while (index2 <= 11);
       let mut local1: &Graphics = &graphics;
      Rectangle rectangle1 = Rectangle::new(100, 498, 300, 14);
      let mut rect1_1: &Rectangle = &rectangle1
      Rectangle rectangle2;
      let mut rect2_1: &Rectangle = &rectangle2
      DrawMod.MakeFullBoxVic2( local1, rect1_1, "OPPONENTS", rect2_1, "");
      txt: String;
      if (Strings.Len(this.game.Data.Designer) < 1)
      {
        txt = "";
      }
      else
      {
        txt = "by: " + this.game.Data.Designer;
        if (Strings.Len(this.game.Data.Designer2) > 0)
          txt = txt + " & " + this.game.Data.Designer2;
      }
      if (Strings.InStr(this.game.Data.Description, "[tab]") > 0)
      {
        DrawMod.DrawTextVic( graphics, this.game.Data.Name, this.game.VicFont8, 78, 29, this.game.VicColor2, this.game.VicColor1Shade);
        tsubpart =  new MiniMapPartClass(this.game, false, 395, 144);
        this.minimapid = this.AddSubPart( tsubpart, 520, 120, 395, 144, 0);
        DrawMod.DrawRectangle( graphics, 519, 119, 397, 146,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
        let mut num7: i32 =  490 + this.game.EditObj.MiniMap.Width;
        let mut num8: i32 =   Math.Round( (graphics.MeasureString(this.game.Data.Name, this.game.VicFont8).Width + 4f));
        DrawMod.DrawTextVic( graphics, ", " + txt, this.game.VicFont2, 78 + num8, 38, this.game.VicColor2, this.game.VicColor1Shade);
        DrawMod.drawLine( graphics, 82, 58, 78 + num8 - 8, 58,  this.game.VicColor2.R,  this.game.VicColor2.G,  this.game.VicColor2.B,  this.game.VicColor2.A, 2);
        if (Strings.Len(this.game.Data.RuleSetName) > 1)
        {
          tsubpart =  new TextAreaClass(this.game, 430, 18, this.game.VicFont3, "Description", true, this.game.Data.Description, Color.White, tItemSize: 18, tbackbitmap: ( this.OwnBitmap), bbx: 520, bby: 295);
          this.TempText = this.AddSubPart( tsubpart, 520, 295, 430, 342, 0);
        }
        else
        {
          tsubpart =  new TextAreaClass(this.game, 430, 20, this.game.VicFont3, "Description", true, this.game.Data.Description, Color.White, tItemSize: 18, tbackbitmap: ( this.OwnBitmap), bbx: 520, bby: 295);
          this.TempText = this.AddSubPart( tsubpart, 520, 295, 430, 378, 0);
        }
      }
      else
      {
        tsubpart =  new ATTextPartClass(this.game.Data.Name, this.game.VicFont8, 810, 28, true, tBlackBack: true);
        this.TempText2 = this.AddSubPart( tsubpart, 100, 26, 850, 28, 0);
        tsubpart =  new ATTextPartClass(txt, this.game.VicFont3, 810, 14, true, tBlackBack: true);
        this.BRandomID = this.AddSubPart( tsubpart, 100, 53, 850, 14, 0);
        let mut num9: i32 =  0;
        if (num3 == 0)
          num9 = 200;
        DrawMod.DrawPaperSheet( graphics, 519 - num9, 115, 405 + num9, 362);
        tsubpart =  new PaperAreaClass(this.game, 370 + num9, 16,  null, "Description", false, this.game.Data.Description, this.game.VicColor8, tItemSize: 20, tbackbitmap: ( this.OwnBitmap), bbx: (540 - num9), bby: 136);
        this.TempText = this.AddSubPart( tsubpart, 540 - num9, 136, 370 + num9, 342, 0);
        tsubpart =  new MiniMapPartClass(this.game, false, 395, 144);
        this.minimapid = this.AddSubPart( tsubpart, 520, 513, 395, 144, 0);
        DrawMod.DrawRectangle( graphics, 519, 512, 397, 146,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
         let mut local2: &Graphics = &graphics;
        rectangle1 = Rectangle::new(520, 498, 300, 14);
        let mut rect1_2: &Rectangle = &rectangle1
        let mut rect2_2: &Rectangle = &rectangle2
        DrawMod.MakeFullBoxVic2( local2, rect1_2, "MINIMAP", rect2_2, "");
      }
      this.doregimelist();
      tsubpart =  new SteveButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: 20, bby: 710);
      this.cancelID = this.AddSubPart( tsubpart, 20, 710, 35, 35, 1);
      tsubpart =  new TextButtonPartClass("Edit", 70, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 710);
      this.BEditorID = this.AddSubPart( tsubpart, 100, 710, 70, 35, 1);
      tsubpart =  new TextButtonPartClass("Start", 120, tBackbitmap: ( this.OwnBitmap), bbx: 335, bby: 695, theight: 50);
      this.BStartGameID = this.AddSubPart( tsubpart, 335, 695, 120, 50, 1);
      if (Strings.Len(this.game.Data.RuleSetName) > 1)
      {
        SizeF sizeF2 = graphics.MeasureString("Ruleset: " + this.game.Data.RuleSetName, this.game.VicFont3);
        DrawMod.DrawSteveBlock( graphics,  Math.Round(916.0 - ( sizeF2.Width + 10.0)), 682,  Math.Round( (sizeF2.Width + 10f)), 20);
        DrawMod.DrawTextVic2( graphics, "Ruleset: " + this.game.Data.RuleSetName, this.game.VicFont3,  Math.Round(916.0 - ( sizeF2.Width + 5.0)), 687, this.game.VicColor2, this.game.VicColor2Shade);
      }
      if (!this.game.Data.FOWOn)
      {
        tsubpart =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 115);
        this.opt1 = this.AddSubPart( tsubpart, 100, 115, 35, 35, 1);
      }
      else
      {
        tsubpart =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 115);
        this.opt1 = this.AddSubPart( tsubpart, 100, 115, 35, 35, 1);
      }
      tsubpart =  new ATTextPartClass("Fog of War", this.game.VicFont2, 100, 16, false, toutline: true);
      this.txt1 = this.AddSubPart( tsubpart, 140, 122, 100, 16, 0);
      if (!this.game.Data.PasswordsOn)
      {
        tsubpart =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 185);
        this.opt3 = this.AddSubPart( tsubpart, 100, 185, 35, 35, 1);
      }
      else
      {
        tsubpart =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 185);
        this.opt3 = this.AddSubPart( tsubpart, 100, 185, 35, 35, 1);
      }
      tsubpart =  new ATTextPartClass("Passwords", this.game.VicFont2, 100, 16, false, toutline: true);
      this.txt3 = this.AddSubPart( tsubpart, 140, 192, 100, 16, 0);
      if (!this.game.Data.PBEM)
      {
        tsubpart =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 150);
        this.opt5 = this.AddSubPart( tsubpart, 100, 150, 35, 35, 1);
      }
      else
      {
        tsubpart =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 150);
        this.opt5 = this.AddSubPart( tsubpart, 100, 150, 35, 35, 1);
      }
      tsubpart =  new ATTextPartClass("PBEM protection", this.game.VicFont2, 110, 16, false, toutline: true);
      this.txt5 = this.AddSubPart( tsubpart, 140, 157, 110, 16, 0);
      if ( this.game.Data.RuleVar[353] == 0.0)
      {
        if (!this.game.Data.ShrowdOn)
        {
          tsubpart =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: ( byte.MaxValue));
          this.opt2 = this.AddSubPart( tsubpart, 100,  byte.MaxValue, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: ( byte.MaxValue));
          this.opt2 = this.AddSubPart( tsubpart, 100,  byte.MaxValue, 35, 35, 1);
        }
        tsubpart =  new ATTextPartClass("Shroud", this.game.VicFont2, 100, 16, false, toutline: true);
        this.txt2 = this.AddSubPart( tsubpart, 140, 262, 100, 16, 0);
        if (!this.game.Data.ShrowdPeek)
        {
          tsubpart =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 290);
          this.opt7 = this.AddSubPart( tsubpart, 100, 290, 35, 35, 1);
        }
        else
        {
          tsubpart =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 290);
          this.opt7 = this.AddSubPart( tsubpart, 100, 290, 35, 35, 1);
        }
        tsubpart =  new ATTextPartClass("Shroud Peak", this.game.VicFont2, 100, 16, false, toutline: true);
        this.txt7 = this.AddSubPart( tsubpart, 140, 297, 100, 16, 0);
      }
      if (!this.game.Data.DontShowAIMove)
      {
        tsubpart =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 220);
        this.opt9 = this.AddSubPart( tsubpart, 100, 220, 35, 35, 1);
      }
      else
      {
        tsubpart =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 100, bby: 220);
        this.opt9 = this.AddSubPart( tsubpart, 100, 220, 35, 35, 1);
      }
      tsubpart =  new ATTextPartClass("Hide realtime AI", this.game.VicFont2, 110, 16, false, toutline: true);
      this.txt9 = this.AddSubPart( tsubpart, 140, 227, 110, 16, 0);
    }

    pub fn doregimelist()
    {
      if (!this.game.Data.NoPlayChoice)
      {
        let mut tlistselect: i32 =  -1;
        let mut num: i32 =  -1;
        this.RegimeListObj = ATListClass::new();
        if (this.game.Data.RegimeCounter > -1)
        {
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (let mut tdata: i32 =  0; tdata <= regimeCounter; tdata += 1)
          {
            if (!this.game.Data.RegimeObj[tdata].Sleep)
            {
              num += 1;
              name: String = this.game.Data.RegimeObj[tdata].Name;
              if (tdata == this.detailnr)
                tlistselect = num;
              tvalue: String = !this.game.Data.RegimeObj[tdata].Sleep ? (!this.game.Data.RegimeObj[tdata].AI ? "HUMAN" : (this.game.Data.UseAI != 1 ? (this.game.Data.RegimeObj[tdata].ProdBonus > -25 ? (this.game.Data.RegimeObj[tdata].ProdBonus != 0 ? (this.game.Data.RegimeObj[tdata].ProdBonus > 100 ? "AI++" : "AI+") : "AI") : "AI-") : "AI")) : (!this.game.Data.RegimeObj[tdata].AI ? "FIXED HUMAN" : "FIXED AI");
              this.RegimeListObj.add(name, tdata, tvalue);
            }
          }
        }
        if (this.RegimeListId > 0)
        {
          this.SubPartList[this.SubpartNr(this.RegimeListId)].Refresh(this.RegimeListObj, tlistselect);
          this.SubPartFlag[this.SubpartNr(this.RegimeListId)] = true;
        }
        else
        {
          tHeader: String = "OPPONENTS".to_owned();
          if (this.game.Data.NoAIAdvice)
            tHeader = "OPPONENTS (AI blocked)";
          let mut tsubpart: SubPartClass =  new ATListSubPartClass(this.RegimeListObj, 8, 355, tlistselect, this.game, true, tHeader, false, false, tShowPair: true, tValueWidth: 130, tdotopandbottom: false, tbackbitmap: ( this.OwnBitmap), bbx: 95, bby: 513);
          this.RegimeListId = this.AddSubPart( tsubpart, 95, 513, 355, 176, 0);
        }
      }
      else
      {
        if (this.RegimeListId <= 0)
          return;
        this.RemoveSubPart(this.RegimeListId);
      }
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
            if (num1 == this.TempText)
            {
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.RegimeListId)
            {
              let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                if (!this.game.Data.NoAIAdvice)
                {
                  this.detailnr = num2;
                  if (!this.game.Data.RegimeObj[this.detailnr].Sleep)
                  {
                    if (this.game.Data.UseAI == 1)
                    {
                      if (!this.game.Data.RegimeObj[this.detailnr].AI)
                        this.game.Data.RegimeObj[this.detailnr].AI = true;
                      else if (this.game.Data.RegimeObj[this.detailnr].AI)
                        this.game.Data.RegimeObj[this.detailnr].AI = false;
                    }
                    else if (!this.game.Data.RegimeObj[this.detailnr].AI)
                    {
                      this.game.Data.RegimeObj[this.detailnr].AI = true;
                      this.game.Data.RegimeObj[this.detailnr].ProdBonus = 0;
                    }
                    else if (this.game.Data.RegimeObj[this.detailnr].AI & this.game.Data.RegimeObj[this.detailnr].ProdBonus == 0)
                    {
                      this.game.Data.RegimeObj[this.detailnr].AI = true;
                      this.game.Data.RegimeObj[this.detailnr].ProdBonus = 100;
                    }
                    else if (this.game.Data.RegimeObj[this.detailnr].AI & this.game.Data.RegimeObj[this.detailnr].ProdBonus >= 0 & this.game.Data.RegimeObj[this.detailnr].ProdBonus <= 100)
                    {
                      this.game.Data.RegimeObj[this.detailnr].AI = true;
                      this.game.Data.RegimeObj[this.detailnr].ProdBonus = 250;
                    }
                    else if (this.game.Data.RegimeObj[this.detailnr].AI & this.game.Data.RegimeObj[this.detailnr].ProdBonus > 100)
                    {
                      this.game.Data.RegimeObj[this.detailnr].AI = true;
                      this.game.Data.RegimeObj[this.detailnr].ProdBonus = -25;
                    }
                    else
                    {
                      this.game.Data.RegimeObj[this.detailnr].AI = false;
                      this.game.Data.RegimeObj[this.detailnr].ProdBonus = 0;
                    }
                    this.detailnr = num2;
                    this.doregimelist();
                  }
                }
                else
                {
                  let mut num3: i32 =   Interaction.MsgBox( "Scenario is not suitable for playing the AI.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                this.DoStuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.cancelID)
            {
              BitmapStore.ReloadSystemGraphics(this.game.ModSystemGraphicsDirectory);
              this.game.EditObj.ShowInitialMenu = true;
              windowReturnClass.AddCommand(3, 1);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.opt1)
            {
              this.game.Data.FOWOn = !this.game.Data.FOWOn;
              if (this.game.Data.ShrowdOn)
                this.game.Data.FOWOn = true;
              this.DoStuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.opt2)
            {
              if (!this.game.Data.CreatedWithShrowd)
              {
                this.game.Data.ShrowdOn = !this.game.Data.ShrowdOn;
                if (this.game.Data.ShrowdOn)
                  this.game.Data.FOWOn = true;
                if (!this.game.Data.ShrowdOn)
                  this.game.Data.ShrowdPeek = false;
                windowReturnClass.AddCommand(3, 1);
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num4: i32 =   Interaction.MsgBox( "Not possible. This is a random game created with a shroud.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            else if (num1 == this.opt3)
            {
              this.game.Data.PasswordsOn = !this.game.Data.PasswordsOn;
              this.DoStuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.opt5)
            {
              this.game.Data.PBEM = !this.game.Data.PBEM;
              this.game.Data.TerrorMode = false;
              this.DoStuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.opt6)
            {
              let mut aiHelpMove: i32 =  this.game.Data.RegimeObj[0].AIHelpMove;
              let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
              for (let mut index2: i32 =  0; index2 <= regimeCounter; index2 += 1)
              {
                switch (aiHelpMove)
                {
                  case 0:
                    this.game.Data.RegimeObj[index2].AIHelpMove = 20;
                    this.game.Data.RegimeObj[index2].AIHelpCombat = 25;
                    this.game.Data.RegimeObj[index2].AIHelpStrategic = 50;
                    break;
                  case 20:
                    this.game.Data.RegimeObj[index2].AIHelpMove = 30;
                    this.game.Data.RegimeObj[index2].AIHelpCombat = 50;
                    this.game.Data.RegimeObj[index2].AIHelpStrategic = 100;
                    break;
                  case 30:
                    this.game.Data.RegimeObj[index2].AIHelpMove = 40;
                    this.game.Data.RegimeObj[index2].AIHelpCombat = 75;
                    this.game.Data.RegimeObj[index2].AIHelpStrategic = 150;
                    break;
                  case 40:
                    this.game.Data.RegimeObj[index2].AIHelpMove = 50;
                    this.game.Data.RegimeObj[index2].AIHelpCombat = 100;
                    this.game.Data.RegimeObj[index2].AIHelpStrategic = 200;
                    break;
                  case 50:
                    this.game.Data.RegimeObj[index2].AIHelpMove = 0;
                    this.game.Data.RegimeObj[index2].AIHelpCombat = 0;
                    this.game.Data.RegimeObj[index2].AIHelpStrategic = 0;
                    break;
                }
              }
              this.DoStuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.opt9)
            {
              this.game.Data.DontShowAIMove = !this.game.Data.DontShowAIMove;
              this.DoStuff();
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.opt7)
            {
              if (this.game.Data.ShrowdOn)
              {
                this.game.Data.ShrowdPeek = !this.game.Data.ShrowdPeek;
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num5: i32 =   Interaction.MsgBox( "This option can only be activated if map is shrouded.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            else if (num1 == this.opt8)
            {
              if (this.game.Data.PBEM)
              {
                this.game.Data.TerrorMode = !this.game.Data.TerrorMode;
                if (this.game.Data.TerrorMode)
                {
                  let mut num6: i32 =   Interaction.MsgBox( "Be warned that you can only play your turn once with Terror Mode on. After it had been opened once it cannot be reopened again. This is a very safe mode, but if anything goes wrong it will ruin your pbem game because it will not allow you to continue play. Be warned if you play with this ultimate protection mode on.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
              else
              {
                let mut num7: i32 =   Interaction.MsgBox( "This terror mode anti cheat option can only be activated if normal Anti Cheat is already on.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            else
            {
              if (num1 == this.BRandomID)
              {
                windowReturnClass.AddCommand(1, 49);
                windowReturnClass.AddCommand(2, 50);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BStartGameID)
              {
                let mut num8: i32 =  0;
                let mut aiHelpMove: i32 =  this.game.Data.RegimeObj[0].AIHelpMove;
                let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
                for (let mut index3: i32 =  0; index3 <= regimeCounter1; index3 += 1)
                {
                  switch (aiHelpMove)
                  {
                    case 20:
                      this.game.Data.RegimeObj[index3].AIHelpMove = 20;
                      this.game.Data.RegimeObj[index3].AIHelpCombat = 25;
                      this.game.Data.RegimeObj[index3].AIHelpStrategic = 50;
                      break;
                    case 30:
                      this.game.Data.RegimeObj[index3].AIHelpMove = 30;
                      this.game.Data.RegimeObj[index3].AIHelpCombat = 50;
                      this.game.Data.RegimeObj[index3].AIHelpStrategic = 100;
                      break;
                    case 40:
                      this.game.Data.RegimeObj[index3].AIHelpMove = 40;
                      this.game.Data.RegimeObj[index3].AIHelpCombat = 75;
                      this.game.Data.RegimeObj[index3].AIHelpStrategic = 150;
                      break;
                    case 50:
                      this.game.Data.RegimeObj[index3].AIHelpMove = 50;
                      this.game.Data.RegimeObj[index3].AIHelpCombat = 100;
                      this.game.Data.RegimeObj[index3].AIHelpStrategic = 200;
                      break;
                  }
                  if (!this.game.Data.RegimeObj[index3].AI)
                  {
                    this.game.Data.RegimeObj[index3].AIHelpMove = 0;
                    this.game.Data.RegimeObj[index3].AIHelpCombat = 0;
                    this.game.Data.RegimeObj[index3].AIHelpStrategic = 0;
                  }
                }
                let mut regimeCounter2: i32 =  this.game.Data.RegimeCounter;
                for (let mut index4: i32 =  0; index4 <= regimeCounter2; index4 += 1)
                {
                  if (this.game.Data.RegimeObj[index4].AI | this.game.Data.RegimeObj[index4].Sleep)
                    num8 += 1;
                }
                if (num8 == this.game.Data.RegimeCounter + 1 & this.game.Data.CampaignRoom == -1)
                {
                  let mut num9: i32 =   Interaction.MsgBox( "There must be at least 1 human player", Title: ( "Shadow Empire : Planetary Conquest"));
                  if (MsgBoxResult.No == Interaction.MsgBox( "Do you want to proceed anyway?", MsgBoxStyle.YesNo))
                    goto label_157;
                }
                let mut num10: i32 =  0;
                let mut regimeCounter3: i32 =  this.game.Data.RegimeCounter;
                for (let mut index5: i32 =  0; index5 <= regimeCounter3; index5 += 1)
                {
                  if (this.game.Data.RegimeObj[index5].AI)
                    num10 += 1;
                }
                if (this.game.Data.NoAIAdvice & num10 > 0)
                {
                  let mut num11: i32 =   Interaction.MsgBox( "The scenario creator has allowed no AIs in this scenario. You are not allowed to play for your own good.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  this.game.Data.Round = 0;
                  let mut mapWidth: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
                  for (let mut index6: i32 =  0; index6 <= mapWidth; index6 += 1)
                  {
                    let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
                    for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
                    {
                      let mut regimeCounter4: i32 =  this.game.Data.RegimeCounter;
                      for (let mut Index: i32 =  0; Index <= regimeCounter4; Index += 1)
                      {
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index6, index7].set_LastLT(Index, -1);
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index6, index7].set_LastSpr(Index, -1);
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index6, index7].set_LastReg(Index, -1);
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index6, index7].set_SeeNow(Index, 0);
                      }
                    }
                  }
                  let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                  for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
                  {
                    if (!this.game.Data.UnitObj[unr].IsHQ && this.game.Data.UnitObj[unr].Supply > this.game.HandyFunctionsObj.UnitSupplyStore(unr))
                      this.game.Data.UnitObj[unr].Supply = this.game.HandyFunctionsObj.UnitSupplyStore(unr);
                  }
                  if (this.game.Data.DoAllied)
                  {
                    let mut regimeCounter5: i32 =  this.game.Data.RegimeCounter;
                    for (let mut index8: i32 =  0; index8 <= regimeCounter5; index8 += 1)
                    {
                      let mut regimeCounter6: i32 =  this.game.Data.RegimeCounter;
                      for (let mut index9: i32 =  0; index9 <= regimeCounter6; index9 += 1)
                      {
                        if (index8 != index9 & this.game.Data.RegimeObj[index8].AI & this.game.Data.RegimeObj[index9].AI)
                        {
                          if ( this.game.Data.RuleVar[524] == 1.0)
                          {
                            this.game.Data.RegimeObj[index8].RegimeRel[index9] = 2;
                            this.game.Data.RegimeObj[index9].RegimeRel[index8] = 2;
                          }
                          else
                          {
                            this.game.Data.RegimeObj[index8].RegimeRel[index9] = 1;
                            this.game.Data.RegimeObj[index9].RegimeRel[index8] = 1;
                          }
                        }
                      }
                    }
                  }
                  if (this.game.Data.Turn > -1)
                  {
                    let mut turn: i32 =  this.game.Data.Turn;
                    for (let mut regnr: i32 =  0; regnr <= turn; regnr += 1)
                    {
                      this.game.ProcessingObj.SetInitialReconAndZOC(regnr);
                      this.game.HandyFunctionsObj.ClearHistory( regnr);
                    }
                  }
                  VBMath.Randomize();
                  this.game.Data.GameID =  Math.Round( (VBMath.Rnd() * 1E+08f));
                  this.game.HandyFunctionsObj.RedimStats();
                  this.game.HandyFunctionsObj.DoResMod();
                  windowReturnClass.AddCommand(3, 4);
                  SoundMod.StopWave();
                }
              }
              else if (num1 == this.BSaveGameID)
              {
                str: String = this.game.HandyFunctionsObj.SaveSomething("Text Files (*.pt2)|*.pt2", "Give save name...", this.game.AppPath + "scenarios\\", false);
                if (Strings.Len(str) < 2)
                {
                  let mut num12: i32 =   Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  this.game.Data.serialize(str);
                  this.RemoveSubPart(this.TempText);
                  let mut tsubpart: SubPartClass =  TextPartClass::new(this.game.Data.Name + " is saved.", this.game.VicFont1, 400, 20, true);
                  this.TempText = this.AddSubPart( tsubpart, 0, 41, 400, 19, 0);
                  windowReturnClass.SetFlag(true);
                }
              }
              else if (num1 == this.BLoadGameID)
              {
                str: String = this.game.HandyFunctionsObj.LoadSomething("Scenario file (*.pt2)|*.pt2|Master File (*.ptmaster)|*.ptmaster|Hidden File (*.pthidden)|*.pthidden", "Pick a scenario to load...", this.game.AppPath + "scenarios\\", false);
                if (File.Exists(str))
                {
                  this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
                  if (this.game.Data.UseAI == 1)
                    this.game.NewAIObj.LastRegime = -1;
                  this.game.SelectX = -1;
                  this.game.SelectY = -1;
                  this.game.Data = DataClass.deserialize(str);
                  if (Strings.Len(this.game.Data.MasterFile) > 0 & this.game.Data.Round == 0)
                    this.game.HandyFunctionsObj.LoadMasterFile(this.game.Data.MasterFile);
                  if (this.game.Data.Round > 0)
                  {
                    BitmapStore.ReloadSystemGraphics(this.game.Data.SystemGfx);
                    this.game.Data.LoadGraphics((Form1) null);
                    this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 210, 115, false);
                    if (!this.game.Data.InTurn)
                    {
                      windowReturnClass.AddCommand(3, 4);
                      this.game.EditObj.Phase = -1;
                    }
                    else
                      windowReturnClass.AddCommand(3, 3);
                    return windowReturnClass;
                  }
                  if (Strings.Len(this.game.Data.LoadPass) > 0)
                  {
                    if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by a load password. Please give it in order to load it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(this.game.Data.LoadPass), false) == 0)
                    {
                      let mut num13: i32 =   Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      let mut num14: i32 =   Interaction.MsgBox( "Wrong Password. You cannot Load this file", Title: ( "Shadow Empire : Planetary Conquest"));
                      this.game.Data = DataClass::new();
                      this.RemoveSubPart(this.TempText);
                      let mut tsubpart: SubPartClass =  TextPartClass::new(this.game.Data.Name + " is loaded instead.", this.game.VicFont1, 400, 20, true);
                      this.TempText = this.AddSubPart( tsubpart, 0, 41, 400, 19, 0);
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                  }
                  BitmapStore.ReloadSystemGraphics(this.game.Data.SystemGfx);
                  this.game.Data.LoadGraphics((Form1) null);
                  this.RemoveSubPart(this.TempText);
                  windowReturnClass.AddCommand(3, 1);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num15: i32 =   Interaction.MsgBox( "File could not be found or op. is cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
              else if (num1 == this.BEditorID)
              {
                let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                for (let mut index10: i32 =  0; index10 <= unitCounter; index10 += 1)
                {
                  this.game.Data.UnitObj[index10].Name = this.game.Data.UnitObj[index10].Name.Replace("3th", "3rd");
                  this.game.Data.UnitObj[index10].Name = this.game.Data.UnitObj[index10].Name.Replace("11st", "11th");
                  this.game.Data.UnitObj[index10].Name = this.game.Data.UnitObj[index10].Name.Replace("12nd", "12th");
                  this.game.Data.UnitObj[index10].Name = this.game.Data.UnitObj[index10].Name.Replace("13rd", "13th");
                }
                if (!this.game.Data.CreatedWithShrowd)
                {
                  if (Strings.Len(this.game.Data.EditPass) > 0)
                  {
                    if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by an edit password. Please give it in order to edit it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(this.game.Data.EditPass), false) == 0)
                    {
                      let mut num16: i32 =   Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      let mut num17: i32 =   Interaction.MsgBox( "Wrong Password. You cannot Edit this file", Title: ( "Shadow Empire : Planetary Conquest"));
                      return windowReturnClass;
                    }
                  }
                  this.game.EditObj.InEditor = true;
                  this.game.SelectX = 0;
                  this.game.SelectY = 0;
                  this.game.EditObj.UnitSelected = -1;
                  this.game.CornerX = 0;
                  this.game.CornerY = 0;
                  this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 210, 115, false);
                  windowReturnClass.AddCommand(3, 2);
                  SoundMod.StopWave();
                }
                else
                {
                  let mut num18: i32 =   Interaction.MsgBox( "Not possible. This is a random game created with a shroud.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
            }
label_157:
            let mut index11: i32 =  0;
            do
            {
              if (this.SubPartID[index1] == this.vari[index11])
              {
                if (this.game.Data.GameSlot[this.game.Data.Variants[index11]] <= 0)
                  this.game.Data.GameSlot[this.game.Data.Variants[index11]] = 1;
                else
                  this.game.Data.GameSlot[this.game.Data.Variants[index11]] = 0;
                if (this.game.Data.VariantEvent[index11] > -1)
                {
                  this.game.FormRef.Cursor = Cursors.WaitCursor;
                  this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.VariantEvent[index11]);
                  this.game.FormRef.Cursor = Cursors.Default;
                }
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
              index11 += 1;
            }
            while (index11 <= 11);
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
