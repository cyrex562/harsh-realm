// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.IntroWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;
// usingSystem.Drawing.Text;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class IntroWindowClass2 : WindowClass
  {
     BStartGameID: i32;
     Bstartgame2id: i32;
     BLoadGameID: i32;
     BSaveGameID: i32;
     BRandomID: i32;
     BEditorID: i32;
     BSimpleId: i32;
     BEditorIDb: i32;
     BSimpleIdb: i32;
     TempText: i32;
     TempText2: i32;
     txt1: i32;
     txt2: i32;
     txt3: i32;
     pass: i32;
     passText: i32;
     title: i32;
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
     opt10: i32;
     cancelID: i32;
     int[] vari;
     int[] varitext;
     ListClass RegimeListObj;
     RegimeListId: i32;
     float tempBlink;
     detailnr: i32;
     selectedid: i32;

    pub IntroWindowClass2( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND1MARC)
    {
      this.vari = new int[12];
      this.varitext = new int[12];
      this.NewGfx = true;
      this.tempBlink = 0.0f;
      this.detailnr = -1;
      this.game.EditObj.PbemTitle = this.game.Data.Name;
      this.selectedid = -1;
      this.game.EditObj.MiniMap = new Bitmap(300, 215, PixelFormat.Format32bppPArgb);
      this.game.EditObj.MiniMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 300, 215, false);
      this.game.EditObj.PbemPrivatePassword = "";
      this.game.Data.DontShowAIMove = this.game.EditObj.dontShowAImoves;
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn && this.game.EditObj.PbemChallengeMiscData.Length >= 60)
      {
        this.game.Data.FOWOn = false;
        if (Conversion.Val(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 41, 1)) == 1.0)
          this.game.Data.FOWOn = true;
        this.game.Data.PasswordsOn = false;
        if (Conversion.Val(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 42, 1)) == 1.0)
          this.game.Data.PasswordsOn = true;
        this.game.Data.PBEM = false;
        if (Conversion.Val(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 43, 1)) == 1.0)
          this.game.Data.PBEM = true;
        this.game.Data.DontShowAIMove = false;
        if (Conversion.Val(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 44, 1)) == 1.0)
          this.game.Data.DontShowAIMove = true;
        let mut index1: i32 =  0;
        do
        {
          if (this.game.Data.Variants[index1] > -1)
          {
            if (Conversion.Val(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 45 + index1, 1)) > 0.0)
              this.game.Data.GameSlot[this.game.Data.Variants[index1]] = 1;
            else
              this.game.Data.GameSlot[this.game.Data.Variants[index1]] = -1;
          }
          index1 += 1;
        }
        while (index1 <= 11);
        let mut num: i32 =  Math.Min(7, this.game.Data.RegimeCounter);
        for (let mut index2: i32 =  0; index2 <= num; index2 += 1)
        {
          if (Conversion.Val(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 57 + index2, 1)) == 0.0)
            this.game.Data.RegimeObj[index2].AI = true;
          else if (Conversion.Val(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 57 + index2, 1)) == 2.0)
          {
            this.game.Data.RegimeObj[index2].AI = false;
            this.game.Data.RegimeObj[index2].PbemPlayer = 1;
          }
          else if (Conversion.Val(Strings.Mid(this.game.EditObj.PbemChallengeMiscData, 57 + index2, 1)) == 3.0)
          {
            this.game.Data.RegimeObj[index2].AI = false;
            this.game.Data.RegimeObj[index2].PbemPlayer = 2;
          }
        }
      }
      let mut num1: i32 =  0;
      let mut num2: i32 =  0;
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.MakingChallenge)
      {
        let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
        num3: i32;
        num4: i32;
        for (let mut index: i32 =  0; index <= regimeCounter1; index += 1)
        {
          if (this.game.Data.RegimeObj[index].PbemPlayer <= 0)
          {
            if (this.game.Data.RegimeObj[index].AI | this.game.Data.RegimeObj[index].Sleep)
            {
              this.game.Data.RegimeObj[index].PbemPlayer = 0;
            }
            else
            {
              num1 += 1;
              if (num1 > 2)
                num1 = 1;
              if (num3 == 0 & num1 == 1)
                num3 = 1;
              if (num4 == 0 & num1 == 2)
                num4 = 1;
              this.game.Data.RegimeObj[index].PbemPlayer = num1;
            }
          }
          if (!this.game.Data.RegimeObj[index].Sleep && !this.game.Data.RegimeObj[index].AI)
            num2 += 1;
        }
        if (num2 <= 1)
        {
          let mut regimeCounter2: i32 =  this.game.Data.RegimeCounter;
          for (let mut index: i32 =  0; index <= regimeCounter2; index += 1)
          {
            if (!this.game.Data.RegimeObj[index].Sleep && this.game.Data.RegimeObj[index].AI & !this.game.Data.NoPlayChoice)
            {
              num2 += 1;
              if (num3 == 0)
              {
                this.game.Data.RegimeObj[index].PbemPlayer = 1;
                num3 = 1;
                this.game.Data.RegimeObj[index].AI = false;
              }
              if (num4 == 0)
              {
                this.game.Data.RegimeObj[index].PbemPlayer = 2;
                num4 = 1;
                this.game.Data.RegimeObj[index].AI = false;
                break;
              }
              break;
            }
          }
        }
        if (num2 <= 1)
        {
          let mut regimeCounter3: i32 =  this.game.Data.RegimeCounter;
          for (let mut index: i32 =  0; index <= regimeCounter3; index += 1)
          {
            if (!this.game.Data.RegimeObj[index].Sleep && this.game.Data.RegimeObj[index].AI & !this.game.Data.NoPlayChoice)
            {
              let mut num5: i32 =  num2 + 1;
              if (num3 == 0)
              {
                this.game.Data.RegimeObj[index].PbemPlayer = 1;
                this.game.Data.RegimeObj[index].AI = false;
              }
              if (num4 == 0)
              {
                this.game.Data.RegimeObj[index].PbemPlayer = 2;
                this.game.Data.RegimeObj[index].AI = false;
                break;
              }
              break;
            }
          }
        }
      }
      this.DoStuff();
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (nr == 40)
      {
        this.SubPartList[this.SubpartNr(this.TempText)].ShiftDown();
        this.SubPartFlag[this.SubpartNr(this.TempText)] = true;
        windowReturnClass1.SetFlag(true);
      }
      if (nr == 38)
      {
        this.SubPartList[this.SubpartNr(this.TempText)].ShiftUp();
        this.SubPartFlag[this.SubpartNr(this.TempText)] = true;
        windowReturnClass1.SetFlag(true);
      }
      if (nr == 37)
      {
        this.SubPartList[this.SubpartNr(this.TempText)].ShiftLeft();
        this.SubPartFlag[this.SubpartNr(this.TempText)] = true;
        windowReturnClass1.SetFlag(true);
      }
      if (nr == 39)
      {
        this.SubPartList[this.SubpartNr(this.TempText)].ShiftRight();
        this.SubPartFlag[this.SubpartNr(this.TempText)] = true;
        windowReturnClass1.SetFlag(true);
      }
      if (nr != 27)
        return windowReturnClass1;
      windowReturnClass2: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.cancelID)] + 1, this.SubPartY[this.SubpartNr(this.cancelID)] + 1, 1);
      windowReturnClass2.SetFlag(true);
      return windowReturnClass2;
    }

    pub fn DoStuff()
    {
      SizeF sizeF1 = SizeF::new();
      str: String;
      if (this.pass > 0)
        str = this.SubPartList[this.SubpartNr(this.pass)].GetText();
      if (this.pass > 0)
      {
        this.RemoveSubPart(this.pass);
        this.pass = 0;
      }
      if (this.title > 0)
      {
        this.RemoveSubPart(this.title);
        this.title = 0;
      }
      if (this.passText > 0)
        this.RemoveSubPart(this.passText);
      if (this.TempText > 0)
        this.RemoveSubPart(this.TempText);
      if (this.TempText2 > 0)
        this.RemoveSubPart(this.TempText2);
      if (this.BStartGameID > 0)
        this.RemoveSubPart(this.BStartGameID);
      if (this.Bstartgame2id > 0)
        this.RemoveSubPart(this.Bstartgame2id);
      if (this.BRandomID > 0)
        this.RemoveSubPart(this.BRandomID);
      if (this.BLoadGameID > 0)
        this.RemoveSubPart(this.BLoadGameID);
      if (this.BEditorID > 0)
        this.RemoveSubPart(this.BEditorID);
      if (this.BSimpleId > 0)
        this.RemoveSubPart(this.BSimpleId);
      if (this.BEditorIDb > 0)
        this.RemoveSubPart(this.BEditorIDb);
      if (this.BSimpleIdb > 0)
        this.RemoveSubPart(this.BSimpleIdb);
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
      if (this.opt10 > 0)
        this.RemoveSubPart(this.opt10);
      let mut index1: i32 =  0;
      do
      {
        if (this.vari[index1] > 0)
          this.RemoveSubPart(this.vari[index1]);
        if (this.varitext[index1] > 0)
          this.RemoveSubPart(this.varitext[index1]);
        index1 += 1;
      }
      while (index1 <= 11);
      if (this.cancelID > 0)
        this.RemoveSubPart(this.cancelID);
      this.ClearMouse();
      this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND1MARC);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.MakingChallenge)
      {
        DrawMod.DrawTextColouredMarc( graphics, "SCENARIO:", this.game.MarcFont1, 175, 27, Color.White);
        let mut tsubpart: SubPartClass =  new InputTextClass(this.game.EditObj.PbemTitle, this.game.MarcFont4, 200, 20, false, 50, true);
        this.title = this.AddSubPart( tsubpart, 320, 34, 200, 20, 0);
      }
      else
      {
        DrawMod.DrawTextColouredMarc( graphics, "SCENARIO:", this.game.MarcFont1, 175, 27, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, Strings.UCase(this.game.Data.Name), this.game.MarcFont2, 315, 27, Color.White);
      }
      if (Strings.Len(this.game.Data.Designer) >= 1)
      {
        tstring: String = this.game.Data.Designer;
        if (Strings.Len(this.game.Data.Designer2) > 0)
          tstring = tstring + " & " + this.game.Data.Designer2;
        SizeF sizeF2 = graphics.MeasureString("by: " + tstring, Font::new("Arial Black", 19f, FontStyle.Regular, GraphicsUnit.Pixel));
        let mut x: i32 =   Math.Round( (950f - sizeF2.Width));
        DrawMod.DrawTextColouredMarc( graphics, "by:", this.game.MarcFont1, x, 27, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, tstring, this.game.MarcFont2, x + 50, 27, Color.White);
        Rectangle trect = Rectangle::new(x, 27,  Math.Round( sizeF2.Width),  Math.Round( sizeF2.Height));
        this.AddMouse( trect, "", "The name of the designer of this scenario");
        if (this.game.Data.scenarioVersion.Length > 0)
        {
          DrawMod.DrawTextColouredMarc( graphics, "SCN VERSION: ", this.game.MarcFont14, 181, 65, Color.FromArgb( byte.MaxValue, 240, 240, 240));
          DrawMod.DrawTextColouredMarc( graphics, this.game.Data.scenarioVersion.ToUpper(), this.game.MarcFont14, 260, 65, Color.FromArgb( byte.MaxValue, 220, 220, 220));
        }
      }
      DrawMod.DrawBlock( graphics, 180, 60, 775, 3,  this.game.MarcCol4.R,  this.game.MarcCol4.G,  this.game.MarcCol4.B,  this.game.MarcCol4.A);
       let mut local1: &Graphics = &graphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(this.game.LOGOFLATTINY);
       let mut local2: &Bitmap = &bitmap;
      DrawMod.DrawSimple( local1,  local2, 20, 21);
      if (this.game.Data.CampaignRoom == -1)
      {
        DrawMod.DrawSimple( graphics,  this.game.EditObj.MiniMap, 620, 458);
        DrawMod.DrawBlockGradient2( graphics, 620, 458, 299, 214, this.game.MarcCol1, this.game.MarcCol2);
        DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  graphics, 620, 458, 300, 215, -1, -1);
        DrawMod.DrawTextColouredMarc( graphics, "MAP", this.game.MarcFont4, 630, 437, Color.White);
      }
      let mut num1: i32 =  490 + this.game.EditObj.MiniMap.Width;
      if (this.game.Data.UseAI >= 1 & !this.game.Data.NoAIAdvice & this.game.Data.RegimeCounter > -1)
      {
        if (this.game.Data.UseAI == 1)
          DrawMod.DrawTextColouredMarc( graphics, "AI (DC1) SETTINGS", this.game.MarcFont1, 90, 120, Color.White);
        if (this.game.Data.UseAI == 2)
          DrawMod.DrawTextColouredMarc( graphics, "AI SETTINGS", this.game.MarcFont1, 90, 120, Color.White);
        DrawMod.DrawBlock( graphics, 90, 150, 250, 2,  this.game.MarcCol3.R,  this.game.MarcCol3.G,  this.game.MarcCol3.B,  this.game.MarcCol3.A);
        DrawMod.DrawBlockGradient2( graphics, 140, 158, 199, 79, this.game.MarcCol1, this.game.MarcCol2);
        DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  graphics, 140, 158, 200, 80, -1, -1);
        if (this.game.Data.Product == 6)
        {
          if (this.game.Data.RegimeObj[0].AIHelpStrategic == 0)
            DrawMod.DrawTextColouredMarc( graphics, "NORMAL AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
          else if (this.game.Data.RegimeObj[0].AIHelpStrategic == 50)
            DrawMod.DrawTextColouredMarc( graphics, "CHALLENGING AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
          else if (this.game.Data.RegimeObj[0].AIHelpStrategic == 100)
            DrawMod.DrawTextColouredMarc( graphics, "HARD AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
          else if (this.game.Data.RegimeObj[0].AIHelpStrategic == 150)
            DrawMod.DrawTextColouredMarc( graphics, "VERY HARD AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
          else if (this.game.Data.RegimeObj[0].AIHelpStrategic == 200)
            DrawMod.DrawTextColouredMarc( graphics, "SUPER HARD AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
        }
        else if (this.game.Data.RegimeObj[0].AIHelpMove == 0)
          DrawMod.DrawTextColouredMarc( graphics, "NORMAL AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
        else if (this.game.Data.RegimeObj[0].AIHelpMove == 20)
          DrawMod.DrawTextColouredMarc( graphics, "CHALLENGING AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
        else if (this.game.Data.RegimeObj[0].AIHelpMove == 30)
          DrawMod.DrawTextColouredMarc( graphics, "HARD AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
        else if (this.game.Data.RegimeObj[0].AIHelpMove == 40)
          DrawMod.DrawTextColouredMarc( graphics, "VERY HARD AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
        else if (this.game.Data.RegimeObj[0].AIHelpMove == 50)
          DrawMod.DrawTextColouredMarc( graphics, "SUPER HARD AI DIFFICULTY", this.game.MarcFont5, 150, 170, Color.White);
        let mut tsubpart1: SubPartClass =  new MarcButtonPartClass(this.game.MARCARROW, tBackbitmap: ( this.OwnBitmap), bbx: 90, bby: 162);
        this.opt6 = this.AddSubPart( tsubpart1, 90, 162, 35, 35, 1);
        let mut tsubpart2: SubPartClass =  new MarcButtonPartClass(this.game.MARCARROW, tBackbitmap: ( this.OwnBitmap), bbx: 90, bby: 202);
        this.opt4 = this.AddSubPart( tsubpart2, 90, 202, 35, 35, 1);
        if (this.game.Data.RegimeObj[0].ProdBonus == 0)
          DrawMod.DrawTextColouredMarc( graphics, "FAST AI SPEED", this.game.MarcFont5, 150, 207, Color.White);
        else if (this.game.Data.RegimeObj[0].ProdBonus <= 100)
          DrawMod.DrawTextColouredMarc( graphics, "NORMAL AI SPEED", this.game.MarcFont5, 150, 207, Color.White);
        else if (this.game.Data.RegimeObj[0].ProdBonus <= 250)
          DrawMod.DrawTextColouredMarc( graphics, "SLOW AI SPEED", this.game.MarcFont5, 150, 207, Color.White);
      }
      DrawMod.DrawTextColouredMarc( graphics, "SETTINGS", this.game.MarcFont1, 370, 120, Color.White);
      DrawMod.DrawBlock( graphics, 370, 150, 600, 2,  this.game.MarcCol3.R,  this.game.MarcCol3.G,  this.game.MarcCol3.B,  this.game.MarcCol3.A);
      DrawMod.DrawBlockGradient2( graphics, 420, 158, 144, 239, this.game.MarcCol1, this.game.MarcCol2);
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  graphics, 420, 158, 145, 240, -1, -1);
      DrawMod.DrawBlockGradient2( graphics, 620, 158, 144, 239, this.game.MarcCol1, this.game.MarcCol2);
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  graphics, 620, 158, 145, 240, -1, -1);
      DrawMod.DrawBlockGradient2( graphics, 820, 158, 144, 239, this.game.MarcCol1, this.game.MarcCol2);
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  graphics, 820, 158, 145, 240, -1, -1);
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.None & !this.game.EditorBlock)
      {
        if (((this.game.Data.SimpleEditor ? 1 : 0) & 0) != 0)
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("EDIT", 70, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 718, tinactive: true, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.BEditorIDb = this.AddSubPart( tsubpart, 140, 718, 70, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("EDIT", 70, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 718, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.BEditorID = this.AddSubPart( tsubpart, 140, 718, 70, 35, 1);
        }
      }
      if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.None && !this.game.EditorBlockSimple)
      {
        if (this.game.Data.SimpleEditor | this.game.SuperAdminRights)
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("SIMPLE EDITOR", 140, tBackbitmap: ( this.OwnBitmap), bbx: 240, bby: 718, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.BSimpleId = this.AddSubPart( tsubpart, 240, 718, 140, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("SIMPLE EDITOR", 140, tBackbitmap: ( this.OwnBitmap), bbx: 240, bby: 718, tinactive: true, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.BSimpleIdb = this.AddSubPart( tsubpart, 240, 718, 140, 35, 1);
        }
      }
      SubPartClass tsubpart3;
      if (this.game.Data.RegimeCounter == -1)
      {
        let mut tsubpart4: SubPartClass =  new MarcButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: 20, bby: 718);
        this.cancelID = this.AddSubPart( tsubpart4, 20, 718, 35, 35, 1);
      }
      else if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.None)
      {
        let mut tsubpart5: SubPartClass =  new MarcButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: 20, bby: 718);
        this.cancelID = this.AddSubPart( tsubpart5, 20, 718, 35, 35, 1);
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("START", 120, tBackbitmap: ( this.OwnBitmap), bbx: 800, bby: 710, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
        this.BStartGameID = this.AddSubPart( tsubpart6, 800, 710, 120, 50, 1);
      }
      else if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn)
      {
        let mut tsubpart7: SubPartClass =  new MarcButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: 20, bby: 710);
        this.cancelID = this.AddSubPart( tsubpart7, 20, 710, 35, 35, 1);
        let mut tsubpart8: SubPartClass =  new TextButtonPartClass("START PBEM++", 240, tBackbitmap: ( this.OwnBitmap), bbx: 680, bby: 710, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
        this.BStartGameID = this.AddSubPart( tsubpart8, 680, 710, 240, 50, 1);
      }
      else if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.MakingChallenge)
      {
        DrawMod.DrawTextColouredMarc( graphics, "Optional", this.game.MarcFont4, 310, 714, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, "Password:", this.game.MarcFont4, 310, 730, Color.White);
        if (Information.IsNothing( str))
          str = "";
        tsubpart3 =  new InputTextClass(str, this.game.MarcFont4, 240, 36, false, 20, true);
        this.pass = this.AddSubPart( tsubpart3, 400, 714, 240, 36, 0);
        tsubpart3 =  new MarcButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: 20, bby: 710);
        this.cancelID = this.AddSubPart( tsubpart3, 20, 710, 35, 35, 1);
        let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
        num2: i32;
        num3: i32;
        for (let mut index2: i32 =  0; index2 <= regimeCounter; index2 += 1)
        {
          if (!this.game.Data.RegimeObj[index2].AI & !this.game.Data.RegimeObj[index2].Sleep)
          {
            if (this.game.Data.RegimeObj[index2].PbemPlayer == 1)
              num2 = 1;
            if (this.game.Data.RegimeObj[index2].PbemPlayer == 2)
              num3 = 1;
          }
        }
        if (num2 == 1 & num3 == 1)
        {
          tsubpart3 =  new TextButtonPartClass("CHALLENGE", 240, tBackbitmap: ( this.OwnBitmap), bbx: 680, bby: 710, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
          this.BStartGameID = this.AddSubPart( tsubpart3, 680, 710, 240, 50, 1);
        }
        else
        {
          tsubpart3 =  new TextButtonPartClass("CHALLENGE", 240, tBackbitmap: ( this.OwnBitmap), bbx: 680, bby: 710, tinactive: true, theight: 50, usefont: this.game.MarcFont1, useshadow: true, tMarcStyle: true);
          this.Bstartgame2id = this.AddSubPart( tsubpart3, 680, 710, 240, 50, 1);
        }
      }
      else if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting)
      {
        let mut tsubpart9: SubPartClass =  new MarcButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: 20, bby: 710);
        this.cancelID = this.AddSubPart( tsubpart9, 20, 710, 35, 35, 1);
      }
      if (this.game.Data.CampaignRoom == -1)
      {
        tsubpart3 =  new TextAreaClass2(this.game, 440, 12, this.game.MarcFont8, this.game.Data.Description, 17,  this.OwnBitmap, 140, 440, tUseEncy: true);
        this.TempText = this.AddSubPart( tsubpart3, 140, 440, 440, 251, 0);
      }
      else
      {
        tsubpart3 =  new TextAreaClass2(this.game, 740, 12, this.game.MarcFont8, this.game.Data.Description, 17,  this.OwnBitmap, 140, 440, tUseEncy: true);
        this.TempText = this.AddSubPart( tsubpart3, 140, 440, 740, 251, 0);
      }
      if (!this.game.Data.NoPlayChoice)
      {
        DrawMod.DrawTextColouredMarc( graphics, "OPPONENTS", this.game.MarcFont1, 90, 245, Color.White);
        DrawMod.DrawBlock( graphics, 90, 275, 250, 2,  this.game.MarcCol3.R,  this.game.MarcCol3.G,  this.game.MarcCol3.B,  this.game.MarcCol3.A);
        this.doregimelist();
      }
      tsubpart3 =  new MarcRadioPartClass(0, this.game.Data.FOWOn, tBackbitmap: ( this.OwnBitmap), bbx: 380, bby: 168);
      this.opt1 = this.AddSubPart( tsubpart3, 380, 168, 35, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "FOG OF WAR", this.game.MarcFont5, 430, 176, Color.White);
      if (this.game.EditObj.PbemGameSetup != PbemGameSetupPhase.None)
      {
        this.game.Data.PasswordsOn = false;
      }
      else
      {
        tsubpart3 =  new MarcRadioPartClass(0, this.game.Data.PasswordsOn, tBackbitmap: ( this.OwnBitmap), bbx: 380, bby: 208);
        this.opt3 = this.AddSubPart( tsubpart3, 380, 208, 35, 35, 1);
        DrawMod.DrawTextColouredMarc( graphics, "PASSWORDS", this.game.MarcFont5, 430, 216, Color.White);
      }
      tsubpart3 =  new MarcRadioPartClass(0, this.game.Data.PBEM, tBackbitmap: ( this.OwnBitmap), bbx: 380, bby: 248);
      this.opt5 = this.AddSubPart( tsubpart3, 380, 248, 35, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "PBEM PROTECTION", this.game.MarcFont5, 430, 256, Color.White);
      if ( this.game.Data.RuleVar[353] == 0.0)
      {
        tsubpart3 =  new MarcRadioPartClass(0, this.game.Data.ShrowdOn, tBackbitmap: ( this.OwnBitmap), bbx: 380, bby: 328);
        this.opt2 = this.AddSubPart( tsubpart3, 380, 328, 35, 35, 1);
        DrawMod.DrawTextColouredMarc( graphics, "SHROUD", this.game.MarcFont5, 430, 336, Color.White);
        tsubpart3 =  new MarcRadioPartClass(0, this.game.Data.ShrowdPeek, tBackbitmap: ( this.OwnBitmap), bbx: 380, bby: 368);
        this.opt7 = this.AddSubPart( tsubpart3, 380, 368, 35, 35, 1);
        DrawMod.DrawTextColouredMarc( graphics, "SHROUD PEEK", this.game.MarcFont5, 430, 376, Color.White);
      }
      tsubpart3 =  new MarcRadioPartClass(0, this.game.Data.DontShowAIMove, tBackbitmap: ( this.OwnBitmap), bbx: 380, bby: 288);
      this.opt9 = this.AddSubPart( tsubpart3, 380, 288, 35, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "HIDE AI MOVES", this.game.MarcFont5, 430, 296, Color.White);
      let mut index3: i32 =  0;
      do
      {
        if (this.game.Data.Variants[index3] > -1)
        {
          String1: String = this.game.Data.GameSlotName[this.game.Data.Variants[index3]];
          tDescript: String;
          if (Strings.InStr(String1, ",") > 0)
          {
            strArray: Vec<String> = String1.Split(',');
            String1 = strArray[0];
            tDescript = strArray[1];
          }
          if (index3 <= 5)
          {
            if (this.game.Data.GameSlot[this.game.Data.Variants[index3]] <= 0)
            {
              int[] vari = this.vari;
              let mut index4: i32 =  index3;
              tsubpart3 =  new MarcRadioPartClass(0, false, tDescript,  this.OwnBitmap, 580, 164 + index3 * 40);
              let mut num4: i32 =  this.AddSubPart( tsubpart3, 580, 164 + index3 * 40, 35, 35, 1);
              vari[index4] = num4;
            }
            else
            {
              int[] vari = this.vari;
              let mut index5: i32 =  index3;
              tsubpart3 =  new MarcRadioPartClass(0, true, tDescript,  this.OwnBitmap, 580, 164 + index3 * 40);
              let mut num5: i32 =  this.AddSubPart( tsubpart3, 580, 164 + index3 * 40, 35, 35, 1);
              vari[index5] = num5;
            }
            DrawMod.DrawTextColouredMarc( graphics, Strings.UCase(String1), this.game.MarcFont5, 630, 176 + index3 * 40, Color.White);
          }
          else
          {
            if (this.game.Data.GameSlot[this.game.Data.Variants[index3]] <= 0)
            {
              int[] vari = this.vari;
              let mut index6: i32 =  index3;
              tsubpart3 =  new MarcRadioPartClass(0, false, tDescript,  this.OwnBitmap, 780, 164 + (index3 - 6) * 40);
              let mut num6: i32 =  this.AddSubPart( tsubpart3, 780, 164 + (index3 - 6) * 40, 35, 35, 1);
              vari[index6] = num6;
            }
            else
            {
              int[] vari = this.vari;
              let mut index7: i32 =  index3;
              tsubpart3 =  new MarcRadioPartClass(0, true, tDescript,  this.OwnBitmap, 780, 164 + (index3 - 6) * 40);
              let mut num7: i32 =  this.AddSubPart( tsubpart3, 780, 164 + (index3 - 6) * 40, 35, 35, 1);
              vari[index7] = num7;
            }
            DrawMod.DrawTextColouredMarc( graphics, Strings.UCase(String1), this.game.MarcFont5, 830, 172 + (index3 - 6) * 40, Color.White);
          }
        }
        index3 += 1;
      }
      while (index3 <= 11);
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (!Information.IsNothing( this.game.EditObj.TextInputString) && this.game.EditObj.TextInputString.Length > 0)
      {
        if (this.selectedid == this.pass)
        {
          this.SubPartList[this.SubpartNr(this.selectedid)].Refresh(this.game.EditObj.TextInputString);
          this.SubPartFlag[this.SubpartNr(this.selectedid)] = true;
        }
        if (this.selectedid == this.title)
        {
          this.SubPartList[this.SubpartNr(this.selectedid)].Refresh(this.game.EditObj.TextInputString);
          this.SubPartFlag[this.SubpartNr(this.selectedid)] = true;
        }
        windowReturnClass.SetFlag(true);
      }
      if (this.title > 0)
        this.game.EditObj.PbemTitle = this.SubPartList[this.SubpartNr(this.title)].GetText();
      this.game.EditObj.TextInputString = "";
      return windowReturnClass;
    }

    pub fn doregimelist()
    {
      if (!this.game.Data.NoPlayChoice)
      {
        let mut tlistselect: i32 =  -1;
        let mut num: i32 =  -1;
        this.RegimeListObj = ListClass::new();
        if (this.game.Data.RegimeCounter > -1)
        {
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (let mut tdata: i32 =  0; tdata <= regimeCounter; tdata += 1)
          {
            if (!this.game.Data.RegimeObj[tdata].Sleep)
            {
              num += 1;
              tname: String = Strings.UCase(this.game.Data.RegimeObj[tdata].Name);
              if (tdata == this.detailnr)
                tlistselect = num;
              tvalue: String;
              if (this.game.Data.RegimeObj[tdata].Sleep)
                tvalue = !this.game.Data.RegimeObj[tdata].AI ? "FIXED HUMAN" : "FIXED AI";
              else if (this.game.Data.RegimeObj[tdata].AI)
              {
                tvalue = this.game.Data.UseAI < 1 ? (this.game.Data.RegimeObj[tdata].ProdBonus > 0 ? (this.game.Data.RegimeObj[tdata].ProdBonus > 100 ? "AI++" : "AI+") : "AI") : "AI".to_owned();
              }
              else
              {
                tvalue = "HUMAN".to_owned();
                if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.MakingChallenge | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn)
                {
                  if (this.game.Data.RegimeObj[tdata].PbemPlayer == 1)
                    tvalue = "CHALLENGER".to_owned();
                  if (this.game.Data.RegimeObj[tdata].PbemPlayer == 2)
                  {
                    tvalue = "OPPONENT".to_owned();
                    if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting)
                      tvalue = "YOU".to_owned();
                    if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn && Operators.CompareString(Strings.LCase(this.game.EditObj.PbemUserName), Strings.LCase(this.game.EditObj.PbemPlayer2), false) == 0)
                      tvalue = "YOU".to_owned();
                  }
                }
              }
              this.RegimeListObj.add(tname, tdata, tvalue);
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
          ListClass regimeListObj = this.RegimeListObj;
          let mut game: GameClass = this.game;
           local1: Bitmap =  this.OwnBitmap;
          font: Font =  null;
           local2: Font =  font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(regimeListObj, 6, 250, -1, game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 80, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 90, bby: 285, tMarcStyle: true, overruleFont: ( local2));
          this.RegimeListId = this.AddSubPart( tsubpart, 90, 285, 250, 112, 0);
        }
      }
      else
      {
        if (this.RegimeListId <= 0)
          return;
        this.RemoveSubPart(this.RegimeListId);
      }
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          return;
        }
      }
      let mut subPartCounter: i32 =  this.SubPartCounter;
      for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
      {
        if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
        {
          let mut index2: i32 =  0;
          do
          {
            if (this.SubPartID[index1] == this.vari[index2])
            {
              String1: String = this.game.Data.GameSlotName[this.game.Data.Variants[index2]];
              str: String = "Please check the scenario description for info on this variant.";
              if (Strings.InStr(String1, ",") > 0)
              {
                strArray: Vec<String> = String1.Split(',');
                String1 = strArray[0];
                str = strArray[1];
              }
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = Strings.UCase(String1);
              this.game.EditObj.TipText = str;
            }
            index2 += 1;
          }
          while (index2 <= 11);
          let mut num: i32 =  this.SubPartID[index1];
          if (num == this.cancelID)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "BACK".to_owned();
            this.game.EditObj.TipText = "Return to main menu. [ESC] ";
          }
          else if (num == this.RegimeListId)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "OPPONENTS".to_owned();
            this.game.EditObj.TipText = "Click on a player to toggle it between AI or Human controlled.";
          }
          else if (num == this.opt1)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "FOG OF WAR";
            this.game.EditObj.TipText = "Your units only see nearby enemies and do not have perfect intell on them.";
          }
          else if (num == this.opt3)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "PASSWORDS".to_owned();
            this.game.EditObj.TipText = "You have to give a password before you can open your turn.\r\nMake sure you do not forget it.";
          }
          else if (num == this.opt5)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "PBEM PROTECTION";
            if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.None)
              this.game.EditObj.TipText = "You can only save the game after ending your turn. \r\nAnd reloading a savefile is reported as cheating.";
            else
              this.game.EditObj.TipText = "If turned on nobody can save during turn. Essential for competitive play to avoid cheating.";
          }
          else if (num == this.opt4)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "AI SPEED";
            this.game.EditObj.TipText = "The slower the AI processes the turn the more it will ponder its move.\r\nWhich should result in better moves.";
          }
          else if (num == this.opt9)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "DONT SHOW AI MOVE";
            this.game.EditObj.TipText = "Instead of seeing the AI moves when they come in you see a simple wait screen.\r\nThis increases speed somewhat on single core processors.";
          }
          else if (num == this.opt6)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "AI HELP";
            this.game.EditObj.TipText = "The AI gets an advantage on you.\r\nCurrent bonuses:\r\n";
            if (this.game.Data.RegimeCounter > -1)
            {
              editObj1: EditClass = this.game.EditObj;
              editObj1.TipText = editObj1.TipText + "Movement bonus: " + Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[0].AIHelpMove)) + "%\r\n";
              editObj2: EditClass = this.game.EditObj;
              editObj2.TipText = editObj2.TipText + "Combat bonus: " + Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[0].AIHelpCombat)) + "%\r\n";
              if ( this.game.Data.RuleVar[976] < 1.0 & this.game.Data.Product < 6)
              {
                editObj3: EditClass = this.game.EditObj;
                editObj3.TipText = editObj3.TipText + "Transfer bonus: " + Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[0].AIHelpStrategic)) + "%\r\n";
              }
            }
          }
          else if (num == this.BStartGameID)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "START GAME";
            this.game.EditObj.TipText = "Click to start a new game with the settings as specified in this scenario setup screen.";
          }
          else if (num == this.Bstartgame2id)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "START GAME";
            this.game.EditObj.TipText = "Cannot issue challenge if not both a CHALLENGER player\r\nand OPPONENT specified";
          }
          else if (num == this.BEditorID)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "EDIT GAME";
            this.game.EditObj.TipText = "Click to adjust this scenario in the old complicated editor.";
          }
          else if (num == this.BSimpleId)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "SIMPLE EDIT GAME";
            this.game.EditObj.TipText = "Click to adjust this scenario in the simple editor.";
          }
          else if (num == this.TempText)
            this.SubPartList[index1].HandleToolTip(x - this.SubPartX[index1], y - this.SubPartY[index1]);
        }
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1 & b == 1)
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
              if (num2 > -1 && !(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
              {
                if (!this.game.Data.NoAIAdvice)
                {
                  this.detailnr = num2;
                  if (!this.game.Data.RegimeObj[this.detailnr].Sleep)
                  {
                    if (this.game.Data.UseAI > 0 & this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.MakingChallenge)
                    {
                      if (!this.game.Data.RegimeObj[this.detailnr].AI & this.game.Data.RegimeObj[this.detailnr].PbemPlayer <= 1)
                      {
                        this.game.Data.RegimeObj[this.detailnr].PbemPlayer = 2;
                        this.game.Data.RegimeObj[this.detailnr].AI = false;
                      }
                      else if (!this.game.Data.RegimeObj[this.detailnr].AI & this.game.Data.RegimeObj[this.detailnr].PbemPlayer >= 2)
                      {
                        this.game.Data.RegimeObj[this.detailnr].PbemPlayer = 0;
                        this.game.Data.RegimeObj[this.detailnr].AI = true;
                      }
                      else if (this.game.Data.RegimeObj[this.detailnr].AI)
                      {
                        this.game.Data.RegimeObj[this.detailnr].PbemPlayer = 1;
                        this.game.Data.RegimeObj[this.detailnr].AI = false;
                      }
                    }
                    else if (this.game.Data.UseAI > 0)
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
                    else if (this.game.Data.RegimeObj[this.detailnr].AI & this.game.Data.RegimeObj[this.detailnr].ProdBonus <= 0)
                    {
                      this.game.Data.RegimeObj[this.detailnr].AI = true;
                      this.game.Data.RegimeObj[this.detailnr].ProdBonus = 100;
                    }
                    else if (this.game.Data.RegimeObj[this.detailnr].AI & this.game.Data.RegimeObj[this.detailnr].ProdBonus <= 100)
                    {
                      this.game.Data.RegimeObj[this.detailnr].AI = true;
                      this.game.Data.RegimeObj[this.detailnr].ProdBonus = 250;
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
              if (this.game.EditObj.PbemGameSetup != PbemGameSetupPhase.None)
              {
                if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting)
                  this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.None;
                else if (this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn)
                  this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.CancelPlayTurn;
                else
                  this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.Cancel;
                windowReturnClass.AddCommand(1, 55);
                windowReturnClass.AddCommand(2, 91);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              this.game.EditObj.TutMode = false;
              this.game.EditObj.TutStep = 0;
              this.game.EditObj.TutOrder = -1;
              this.game.EditObj.ShowInitialMenu = true;
              windowReturnClass.AddCommand(3, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.opt1)
            {
              if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
              {
                this.game.Data.FOWOn = !this.game.Data.FOWOn;
                if (this.game.Data.ShrowdOn)
                  this.game.Data.FOWOn = true;
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
            }
            else if (num1 == this.opt2)
            {
              if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
              {
                if (!this.game.Data.CreatedWithShrowd)
                {
                  this.game.Data.ShrowdOn = !this.game.Data.ShrowdOn;
                  if (this.game.Data.ShrowdOn)
                    this.game.Data.FOWOn = true;
                  if (!this.game.Data.ShrowdOn)
                    this.game.Data.ShrowdPeek = false;
                  windowReturnClass.AddCommand(3, 12);
                  windowReturnClass.SetFlag(true);
                }
                else
                {
                  let mut num4: i32 =   Interaction.MsgBox( "Not possible. This is a random game created with a shroud.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
            }
            else if (num1 == this.opt3)
            {
              if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
              {
                this.game.Data.PasswordsOn = !this.game.Data.PasswordsOn;
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
            }
            else if (num1 == this.opt10)
            {
              if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
              {
                this.game.Data.UncertaintyOn = !this.game.Data.UncertaintyOn;
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
            }
            else if (num1 == this.opt5)
            {
              if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
              {
                this.game.Data.PBEM = !this.game.Data.PBEM;
                this.game.Data.TerrorMode = false;
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
            }
            else if (num1 == this.opt4)
            {
              if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
              {
                let mut prodBonus: i32 =  this.game.Data.RegimeObj[0].ProdBonus;
                let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                for (let mut index2: i32 =  0; index2 <= regimeCounter; index2 += 1)
                {
                  switch (prodBonus)
                  {
                    case 0:
                      this.game.Data.RegimeObj[index2].ProdBonus = 100;
                      break;
                    case 100:
                      this.game.Data.RegimeObj[index2].ProdBonus = 250;
                      break;
                    default:
                      this.game.Data.RegimeObj[index2].ProdBonus = 0;
                      break;
                  }
                }
                this.DoStuff();
                windowReturnClass.SetFlag(true);
              }
            }
            else
            {
              if (num1 == this.pass)
              {
                this.selectedid = this.pass;
                this.SubPartList[this.SubpartNr(this.pass)].Descript = "select".to_owned();
                this.SubPartFlag[this.SubpartNr(this.pass)] = true;
                this.SubPartList[this.SubpartNr(this.title)].Descript = "";
                this.SubPartFlag[this.SubpartNr(this.title)] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.title)
              {
                this.selectedid = this.title;
                this.SubPartList[this.SubpartNr(this.pass)].Descript = "";
                this.SubPartFlag[this.SubpartNr(this.pass)] = true;
                this.SubPartList[this.SubpartNr(this.title)].Descript = "select".to_owned();
                this.SubPartFlag[this.SubpartNr(this.title)] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.opt6)
              {
                if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
                {
                  let mut aiHelpMove: i32 =  this.game.Data.RegimeObj[0].AIHelpMove;
                  let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                  for (let mut index3: i32 =  0; index3 <= regimeCounter; index3 += 1)
                  {
                    if (this.game.Data.Product == 6)
                    {
                      switch (aiHelpMove)
                      {
                        case 0:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 15;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 15;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 50;
                          continue;
                        case 15:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 25;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 25;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 100;
                          continue;
                        case 25:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 30;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 40;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 150;
                          continue;
                        case 30:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 35;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 55;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 200;
                          continue;
                        case 35:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 0;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 0;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 0;
                          continue;
                        default:
                          continue;
                      }
                    }
                    else
                    {
                      switch (aiHelpMove)
                      {
                        case 0:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 20;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 10;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 50;
                          continue;
                        case 20:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 30;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 20;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 100;
                          continue;
                        case 30:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 40;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 30;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 150;
                          continue;
                        case 40:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 50;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 40;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 200;
                          continue;
                        case 50:
                          this.game.Data.RegimeObj[index3].AIHelpMove = 0;
                          this.game.Data.RegimeObj[index3].AIHelpCombat = 0;
                          this.game.Data.RegimeObj[index3].AIHelpStrategic = 0;
                          continue;
                        default:
                          continue;
                      }
                    }
                  }
                  this.DoStuff();
                  windowReturnClass.SetFlag(true);
                }
              }
              else if (num1 == this.opt9)
              {
                if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
                {
                  this.game.Data.DontShowAIMove = !this.game.Data.DontShowAIMove;
                  this.game.EditObj.dontShowAImoves = this.game.Data.DontShowAIMove;
                  this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                  this.DoStuff();
                  windowReturnClass.SetFlag(true);
                }
              }
              else if (num1 == this.opt7)
              {
                if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
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
              }
              else if (num1 == this.opt8)
              {
                if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
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
              }
              else if (num1 == this.BStartGameID)
              {
                if (this.game.EditObj.PbemGameSetup != PbemGameSetupPhase.None & this.game.EditObj.PbemGameSetup != PbemGameSetupPhase.PlayTurn)
                {
                  this.game.EditObj.PbemPrivatePassword = this.SubPartList[this.SubpartNr(this.pass)].GetText();
                  if (this.game.EditObj.PbemPrivatePassword.Length > 0 && this.game.EditObj.PbemPrivatePassword.Length < 4 | this.game.EditObj.PbemPrivatePassword.Length > 16)
                  {
                    let mut num8: i32 =   Interaction.MsgBox( "Cancelled challenge because when you set password it must be min 4 and max 16 chars.", Title: ( "Shadow Empire : Planetary Conquest"));
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (this.game.EditObj.PbemTitle.Length < 4 | this.game.EditObj.PbemTitle.Length > 50)
                  {
                    let mut num9: i32 =   Interaction.MsgBox( "Cancelled challenge because name of game must be min 4 and max 50 chars.", Title: ( "Shadow Empire : Planetary Conquest"));
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (this.game.Data.RegimeCounter > 7 | this.game.Data.MasterFile.Length > 1)
                  {
                    let mut num10: i32 =   Interaction.MsgBox( "Cancelled challenge because more then 8 regimes in scenario or because a masterfile is still attached to scenario.", Title: ( "Shadow Empire : Planetary Conquest"));
                    this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.Cancel;
                    windowReturnClass.AddCommand(1, 55);
                    windowReturnClass.AddCommand(2, 91);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  let mut num11: i32 =  0;
                  let mut num12: i32 =  Math.Min(7, this.game.Data.RegimeCounter);
                  for (let mut index4: i32 =  0; index4 <= num12; index4 += 1)
                  {
                    if (this.game.Data.RegimeObj[index4].PbemPlayer <= 0 && !this.game.Data.RegimeObj[index4].AI & !this.game.Data.RegimeObj[index4].Sleep)
                    {
                      num11 += 1;
                      if (num11 > 2)
                        num11 = 1;
                      this.game.Data.RegimeObj[index4].PbemPlayer = num11;
                    }
                  }
                  this.game.EditObj.PbemGameSetup = PbemGameSetupPhase.ChallengeMade;
                  windowReturnClass.AddCommand(1, 55);
                  windowReturnClass.AddCommand(2, 91);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num13: i32 =  0;
                if (this.game.Data.Product != 6)
                {
                  let mut aiHelpMove: i32 =  this.game.Data.RegimeObj[0].AIHelpMove;
                  let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                  for (let mut index5: i32 =  0; index5 <= regimeCounter; index5 += 1)
                  {
                    this.game.Data.RegimeObj[index5].AIHelpMove = 0;
                    this.game.Data.RegimeObj[index5].AIHelpCombat = 0;
                    this.game.Data.RegimeObj[index5].AIHelpStrategic = 0;
                    switch (aiHelpMove)
                    {
                      case 20:
                        this.game.Data.RegimeObj[index5].AIHelpMove = 20;
                        this.game.Data.RegimeObj[index5].AIHelpCombat = 10;
                        this.game.Data.RegimeObj[index5].AIHelpStrategic = 50;
                        break;
                      case 30:
                        this.game.Data.RegimeObj[index5].AIHelpMove = 30;
                        this.game.Data.RegimeObj[index5].AIHelpCombat = 20;
                        this.game.Data.RegimeObj[index5].AIHelpStrategic = 100;
                        break;
                      case 40:
                        this.game.Data.RegimeObj[index5].AIHelpMove = 40;
                        this.game.Data.RegimeObj[index5].AIHelpCombat = 30;
                        this.game.Data.RegimeObj[index5].AIHelpStrategic = 150;
                        break;
                      case 50:
                        this.game.Data.RegimeObj[index5].AIHelpMove = 50;
                        this.game.Data.RegimeObj[index5].AIHelpCombat = 40;
                        this.game.Data.RegimeObj[index5].AIHelpStrategic = 200;
                        break;
                    }
                    if (!this.game.Data.RegimeObj[index5].AI)
                    {
                      this.game.Data.RegimeObj[index5].AIHelpMove = 0;
                      this.game.Data.RegimeObj[index5].AIHelpCombat = 0;
                      this.game.Data.RegimeObj[index5].AIHelpStrategic = 0;
                    }
                  }
                }
                let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
                for (let mut index6: i32 =  0; index6 <= regimeCounter1; index6 += 1)
                {
                  if (this.game.Data.RegimeObj[index6].AI | this.game.Data.RegimeObj[index6].Sleep)
                    num13 += 1;
                }
                if (num13 == this.game.Data.RegimeCounter + 1 & this.game.Data.CampaignRoom == -1)
                {
                  let mut num14: i32 =   Interaction.MsgBox( "There must be at least 1 human player", Title: ( "Shadow Empire : Planetary Conquest"));
                  if (MsgBoxResult.No == Interaction.MsgBox( "Do you want to proceed anyway?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest"))
                    goto label_208;
                }
                if ( this.game.Data.RuleVar[839] == 0.0)
                {
                  let mut num15: i32 =   Interaction.MsgBox( "In order to play a scenario in the VR Editor rulevar(839) 'new GUI' must be set to 1.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  let mut num16: i32 =  0;
                  let mut regimeCounter2: i32 =  this.game.Data.RegimeCounter;
                  for (let mut index7: i32 =  0; index7 <= regimeCounter2; index7 += 1)
                  {
                    if (this.game.Data.RegimeObj[index7].AI)
                      num16 += 1;
                  }
                  if (this.game.Data.NoAIAdvice & num16 > 0)
                  {
                    let mut num17: i32 =   Interaction.MsgBox( "The scenario creator has allowed no AIs in this scenario. You are not allowed to play for your own good.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    this.game.Data.Round = 0;
                    let mut mapWidth: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
                    for (let mut index8: i32 =  0; index8 <= mapWidth; index8 += 1)
                    {
                      let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
                      for (let mut index9: i32 =  0; index9 <= mapHeight; index9 += 1)
                      {
                        let mut regimeCounter3: i32 =  this.game.Data.RegimeCounter;
                        for (let mut Index: i32 =  0; Index <= regimeCounter3; Index += 1)
                        {
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index8, index9].set_LastLT(Index, -1);
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index8, index9].set_LastSpr(Index, -1);
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index8, index9].set_LastReg(Index, -1);
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index8, index9].set_SeeNow(Index, 0);
                        }
                      }
                    }
                    let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                    for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
                    {
                      if (!this.game.Data.UnitObj[unr].IsHQ && this.game.Data.UnitObj[unr].Supply > this.game.HandyFunctionsObj.UnitSupplyStore(unr))
                        this.game.Data.UnitObj[unr].Supply = this.game.HandyFunctionsObj.UnitSupplyStore(unr);
                      if (this.game.Data.Product == 6)
                        this.game.Data.UnitObj[unr].SOInterceptFire = 33;
                    }
                    if (this.game.EditObj.randoallied == 1)
                    {
                      let mut regimeCounter4: i32 =  this.game.Data.RegimeCounter;
                      for (let mut index10: i32 =  0; index10 <= regimeCounter4; index10 += 1)
                      {
                        let mut regimeCounter5: i32 =  this.game.Data.RegimeCounter;
                        for (let mut index11: i32 =  0; index11 <= regimeCounter5; index11 += 1)
                        {
                          if (index10 == index11)
                            this.game.Data.RegimeObj[index10].RegimeRel[index11] = 1;
                          else if (this.game.Data.RegimeObj[index10].AI & this.game.Data.RegimeObj[index11].AI)
                            this.game.Data.RegimeObj[index10].RegimeRel[index11] = 1;
                          else
                            this.game.Data.RegimeObj[index10].RegimeRel[index11] = 0;
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
                    windowReturnClass.AddCommand(3, 13);
                    SoundMod.StopWave();
                  }
                }
              }
              else if (num1 == this.BEditorID)
              {
                if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
                {
                  let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                  for (let mut index12: i32 =  0; index12 <= unitCounter; index12 += 1)
                  {
                    this.game.Data.UnitObj[index12].Name = this.game.Data.UnitObj[index12].Name.Replace("3th", "3rd");
                    this.game.Data.UnitObj[index12].Name = this.game.Data.UnitObj[index12].Name.Replace("11st", "11th");
                    this.game.Data.UnitObj[index12].Name = this.game.Data.UnitObj[index12].Name.Replace("12nd", "12th");
                    this.game.Data.UnitObj[index12].Name = this.game.Data.UnitObj[index12].Name.Replace("13rd", "13th");
                  }
                  if (!this.game.Data.CreatedWithShrowd)
                  {
                    if (Strings.Len(this.game.Data.EditPass) > 0)
                    {
                      if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by an edit password. Please give it in order to edit it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(this.game.Data.EditPass), false) == 0)
                      {
                        let mut num18: i32 =   Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      else
                      {
                        let mut num19: i32 =   Interaction.MsgBox( "Wrong Password. You cannot Edit this file", Title: ( "Shadow Empire : Planetary Conquest"));
                        return windowReturnClass;
                      }
                    }
                    this.game.EditObj.InEditor = true;
                    this.game.SelectX = 0;
                    this.game.SelectY = 0;
                    this.game.EditObj.UnitSelected = -1;
                    this.game.CornerX = 0;
                    this.game.CornerY = 0;
                    this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 200, 150, false);
                    windowReturnClass.AddCommand(3, 2);
                    SoundMod.StopWave();
                  }
                  else
                  {
                    let mut num20: i32 =   Interaction.MsgBox( "Not possible. This is a random game created with a shroud.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                }
              }
              else if (num1 == this.BSimpleId && !(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
              {
                if (!this.game.Data.CreatedWithShrowd)
                {
                  if (Strings.Len(this.game.Data.EditPass) > 0)
                  {
                    if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by an edit password. Please give it in order to edit it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(this.game.Data.EditPass), false) == 0)
                    {
                      let mut num21: i32 =   Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      let mut num22: i32 =   Interaction.MsgBox( "Wrong Password. You cannot Edit this file", Title: ( "Shadow Empire : Planetary Conquest"));
                      return windowReturnClass;
                    }
                  }
                  this.game.EditObj.InEditor = true;
                  this.game.SelectX = 0;
                  this.game.SelectY = 0;
                  this.game.EditObj.UnitSelected = -1;
                  this.game.CornerX = 0;
                  this.game.CornerY = 0;
                  this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 200, 150, false);
                  windowReturnClass.AddCommand(3, 17);
                  SoundMod.StopWave();
                }
                else
                {
                  let mut num23: i32 =   Interaction.MsgBox( "Not possible. This is a random game created with a shroud.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
            }
label_208:
            if (!(this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.Inspecting | this.game.EditObj.PbemGameSetup == PbemGameSetupPhase.PlayTurn))
            {
              let mut tv0: i32 =  0;
              do
              {
                if (this.SubPartID[index1] == this.vari[tv0])
                {
                  if (this.game.Data.GameSlot[this.game.Data.Variants[tv0]] <= 0)
                    this.game.Data.GameSlot[this.game.Data.Variants[tv0]] = 1;
                  else
                    this.game.Data.GameSlot[this.game.Data.Variants[tv0]] = 0;
                  if (this.game.Data.VariantEvent[tv0] > -1)
                  {
                    this.game.FormRef.Cursor = Cursors.WaitCursor;
                    this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.VariantEvent[tv0], tv0);
                    this.game.FormRef.Cursor = Cursors.Default;
                  }
                  this.DoStuff();
                  windowReturnClass.SetFlag(true);
                }
                tv0 += 1;
              }
              while (tv0 <= 11);
            }
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
