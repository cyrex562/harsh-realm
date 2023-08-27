// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PlayResearchWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class PlayResearchWindowClass : WindowClass
  {
     LocNr: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     BAllyId: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     Text4id: i32;
     Text5id: i32;
     text6id: i32;
     text7id: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     OptionsList3Id: i32;
     ListClass OptionsList3Obj;
     OptionsList4Id: i32;
     ListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ListClass OptionsList5Obj;
     OptionsList6Id: i32;
     ListClass OptionsList6Obj;
     detailnr: i32;
     detailnr2: i32;
     detailnr3: i32;
     peoplenr: i32;
     but1id: i32;
     but1textid: i32;
     regnr: i32;
     pplnr: i32;
     pregnr: i32;
     SimpleList SL;
     int[] minicard;
     tempbmp: Vec<Bitmap>;
     main1: i32;
     main2: i32;
     main3: i32;
     main4: i32;
     main5: i32;
     mainnr: i32;
     mainx: i32;
     dodetailnr: i32;
     DateTime lasttime;

    pub PlayResearchWindowClass( tGame: GameClass, tempInt: i32)
      : base( tGame, 1024, 768, 7)
    {
      this.minicard = new int[65];
      this.tempbmp = new Bitmap[65];
      this.game.EditObj.DoCardSlot = -1;
      this.game.EditObj.HandCard = -1;
      this.remainderofnew();
    }

    pub PlayResearchWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND3MARC)
    {
      this.minicard = new int[65];
      this.tempbmp = new Bitmap[65];
      this.game.EditObj.DoCardSlot = -1;
      this.game.EditObj.HandCard = -1;
      this.remainderofnew();
    }

    pub PlayResearchWindowClass( tGame: GameClass, bool Marc)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND1MARC)
    {
      this.minicard = new int[65];
      this.tempbmp = new Bitmap[65];
      this.game.EditObj.DoCardSlot = -1;
      this.game.EditObj.HandCard = -1;
      this.remainderofnew();
    }

    pub fn remainderofnew()
    {
      this.SL = SimpleList::new();
      this.regnr = this.game.Data.Turn;
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.peoplenr = -1;
      this.dodetailnr = -1;
      this.mainnr = 2;
      this.mainnr =  this.game.Data.RuleVar[501] >= 1.0 ? ( this.game.Data.RuleVar[502] >= 1.0 ? 4 : ( this.game.Data.RuleVar[510] >= 1.0 ? 2 : 2)) : 5;
      if (this.game.Data.CampaignRoom > -1)
      {
        this.mainnr = 2;
        this.game.EditObj.DoCardSlot = this.game.Data.CampaignRoom;
      }
      this.regnr = this.game.Data.Turn;
      this.pplnr = this.game.Data.RegimeObj[this.regnr].People;
      this.pregnr = this.game.Data.Turn;
      this.game.EditObj.AreaSlot = -1;
      this.game.EditObj.AreaValue = -1;
      this.game.EditObj.AreaX = -1;
      this.game.EditObj.AreaY = -1;
      this.game.EditObj.AreaMap = -1;
      this.domain();
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      TimeSpan timeSpan = DateAndTime.Now.Subtract(this.lasttime);
      if (timeSpan.Milliseconds + timeSpan.Seconds * 1000 > 100 | this.game.EditObj.DoCardSlot > -1)
      {
        this.lasttime = DateAndTime.Now;
        if (this.game.EditObj.DoCardSlot > -1)
        {
          if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaSlot > -1)
          {
            this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
            this.game.EditObj.AreaSlot = this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaSlot;
            this.game.EditObj.AreaValue = this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaValue;
            this.game.EditObj.PopupValue = 1;
            windowReturnClass1.AddCommand(5, 10);
            this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
          if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].UnitSelect)
          {
            this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
            this.game.EditObj.PopupValue = 3;
            windowReturnClass1.AddCommand(5, 10);
            this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
          let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
          this.game.ProcessingObj.PlayCard(this.game.EditObj.DoCardSlot);
          if (Strings.Len(this.game.Data.LoadGame) > 0)
          {
            this.game.FormRef.Cursor = Cursors.WaitCursor;
            Form formRef =  this.game.FormRef;
            this.game.HandyFunctionsObj.LoadGameNow();
            this.game.FormRef = (Form1) formRef;
            this.game.FormRef.Cursor = Cursors.Default;
            windowReturnClass1.AddCommand(3, 4);
            return windowReturnClass1;
          }
          let mut Number: i32 =  0;
          let mut locCounter: i32 =  this.game.Data.LocCounter;
          for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
          {
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
            {
              let mut index: i32 =  0;
              do
              {
                if (this.game.Data.LocObj[locnr].Production[index] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index]).result)
                {
                  Number += 1;
                  this.game.Data.LocObj[locnr].Production[index] = -1;
                  this.game.Data.LocObj[locnr].ProdPointRemainder[index] = 0;
                  this.game.Data.LocObj[locnr].ProdPercent[index] = 0;
                }
                index += 1;
              }
              while (index <= 3);
            }
          }
          if (Number > 0)
          {
            let mut num: i32 =   Interaction.MsgBox( (Conversion.Str( Number) + " production lines have been cancelled due to this action card being played."), Title: ( "Shadow Empire : Planetary Conquest"));
          }
          if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
          {
            this.game.EditObj.PopupValue = 0;
            this.game.EditObj.FromMessage = messCounter + 1;
            windowReturnClass1.AddCommand(5, 10);
            this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
        else if (Strings.Len(this.game.Data.LoadGame) > 0)
        {
          this.game.FormRef.Cursor = Cursors.WaitCursor;
          formRef: Form1 = this.game.FormRef;
          this.game.HandyFunctionsObj.LoadGameNow();
          this.game.FormRef = formRef;
          this.game.FormRef.Cursor = Cursors.Default;
          this.game.FormRef.StoredScreeny = (ScreenClass) null;
          windowReturnClass2: WindowReturnClass = WindowReturnClass::new();
          windowReturnClass2.AddCommand(3, 4);
          windowReturnClass2.SetFlag(true);
          return windowReturnClass2;
        }
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

     void domain()
    {
      if (this.main1 > 0)
        this.RemoveSubPart(this.main1);
      if (this.main2 > 0)
        this.RemoveSubPart(this.main2);
      if (this.main3 > 0)
        this.RemoveSubPart(this.main3);
      if (this.main4 > 0)
        this.RemoveSubPart(this.main4);
      if (this.main5 > 0)
        this.RemoveSubPart(this.main5);
      if (this.mainx > 0)
        this.RemoveSubPart(this.mainx);
      if (this.but1id > 0)
        this.RemoveSubPart(this.but1id);
      if (this.but1textid > 0)
        this.RemoveSubPart(this.but1textid);
      if (this.game.EditObj.DoCardSlot > -1)
        return;
      if (this.game.Data.CampaignRoom > -1)
      {
        if ( this.game.Data.RuleVar[839] == 0.0)
        {
          if (this.game.EditObj.CampaignRoomBitmap > -1)
            this.NewBackGroundAndClearAll(1024, 768, this.game.Data.EventPicNr[this.game.EditObj.CampaignRoomBitmap]);
          else
            this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND3MARC);
        }
        else
          this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND1MARC);
      }
      else
        this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND3MARC);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      if (this.game.Data.CampaignRoom > -1 & Strings.Len(this.game.EditObj.CampaignRoomTitle) > 0)
      {
        if ( this.game.Data.RuleVar[839] == 1.0)
        {
          DrawMod.DrawTextColouredMarc( graphics, Strings.UCase(this.game.EditObj.CampaignRoomTitle), this.game.MarcFont2, 20, 15, Color.White);
          DrawMod.DrawBlock( graphics, 20, 45, 500, 3,  this.game.MarcCol4.R,  this.game.MarcCol4.G,  this.game.MarcCol4.B,  this.game.MarcCol4.A);
        }
        else
        {
          DrawMod.DrawSteveBlock( graphics, 200, 15, 700, 27);
          DrawMod.DrawBlock( graphics, 20, 15, 900, 27, 0, 0, 0,  byte.MaxValue);
          DrawMod.DrawSteveBlock( graphics, 20, 15, 900, 27);
          DrawMod.DrawTextColoured( graphics, this.game.EditObj.CampaignRoomTitle, Font::new("Arial Black", 17f, FontStyle.Regular, GraphicsUnit.Pixel), 20, 15, Color.White);
        }
      }
      else
      {
        DrawMod.DrawSteveBlock( graphics, 200, 15, 700, 27);
        DrawMod.DrawTextColoured( graphics, "Decision Room", Font::new("Arial Black", 19f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 15, Color.White);
      }
      if (this.game.Data.CampaignRoom == -1)
      {
        let mut tsubpart: SubPartClass =  new SteveButtonPartClass(this.game.BACKBUTTON, tBackbitmap: ( this.OwnBitmap), bbx: 25, bby: 700);
        this.but1id = this.AddSubPart( tsubpart, 25, 700, 35, 35, 1);
      }
      let mut num1: i32 =  200;
      if (Strings.Len(this.game.EditObj.CampaignRoomTitle) > 0)
        num1 = 25;
      usefont: Font;
      bool flag;
      if ( this.game.Data.RuleVar[839] == 1.0)
      {
        usefont = this.game.MarcFont4;
        flag = true;
      }
      else
      {
        usefont =  null;
        flag = false;
      }
      if (!(this.game.Data.CampaignRoom > -1 &  this.game.Data.RuleVar[839] == 1.0))
      {
        SubPartClass tsubpart1;
        if ( this.game.Data.RuleVar[502] == 0.0)
        {
          buttontext: String = "Cards";
          if ( this.game.Data.RuleVar[839] == 1.0)
            buttontext = Strings.UCase(buttontext);
          if (this.mainnr != 2)
          {
            let mut tsubpart2: SubPartClass =  new TextButtonPartClass(buttontext, 150, "Click to see your cards",  this.OwnBitmap, num1, 60, usefont: usefont, useshadow: flag, tMarcStyle: flag);
            this.main2 = this.AddSubPart( tsubpart2, num1, 60, 150, 35, 1);
          }
          else
          {
            tsubpart1 =  new TextButtonPartClass(buttontext, 150, "Your viewing your cards already.",  this.OwnBitmap, num1, 60, tred: (!flag), usefont: usefont, useshadow: flag, tMarcStyle: flag);
            this.mainx = this.AddSubPart( tsubpart1, num1, 60, 150, 35, 1);
          }
          num1 += 160;
          if ( this.game.Data.RuleVar[510] == 0.0)
          {
            if (this.mainnr != 3)
            {
              tsubpart1 =  new TextButtonPartClass("Active Cards", 150, tBackbitmap: ( this.OwnBitmap), bbx: num1, bby: 60, usefont: usefont, useshadow: flag, tMarcStyle: flag);
              this.main3 = this.AddSubPart( tsubpart1, num1, 60, 150, 35, 1);
            }
            else
            {
              tsubpart1 =  new TextButtonPartClass("Active Cards", 150, tBackbitmap: ( this.OwnBitmap), bbx: num1, bby: 60, tred: (!flag), usefont: usefont, useshadow: flag, tMarcStyle: flag);
              this.mainx = this.AddSubPart( tsubpart1, num1, 60, 150, 35, 1);
            }
            num1 += 160;
          }
        }
        buttontext1: String = "Reports";
        if ( this.game.Data.RuleVar[839] == 1.0)
          buttontext1 = Strings.UCase(buttontext1);
        if (this.mainnr != 4)
        {
          tsubpart1 =  new TextButtonPartClass(buttontext1, 150, "Click to view your reports.",  this.OwnBitmap, num1, 60, usefont: usefont, useshadow: flag, tMarcStyle: flag);
          this.main4 = this.AddSubPart( tsubpart1, num1, 60, 150, 35, 1);
        }
        else
        {
          tsubpart1 =  new TextButtonPartClass(buttontext1, 150, "Your currently viewing your reports already.",  this.OwnBitmap, num1, 60, tred: (!flag), usefont: usefont, useshadow: flag, tMarcStyle: flag);
          this.mainx = this.AddSubPart( tsubpart1, num1, 60, 150, 35, 1);
        }
        let mut num2: i32 =  num1 + 160;
        if ( this.game.Data.RuleVar[501] < 1.0)
        {
          if (this.mainnr != 5)
          {
            tsubpart1 =  new TextButtonPartClass("Research", 150, tBackbitmap: ( this.OwnBitmap), bbx: num2, bby: 60, usefont: usefont, useshadow: flag, tMarcStyle: flag);
            this.main5 = this.AddSubPart( tsubpart1, num2, 60, 150, 35, 1);
          }
          else
          {
            tsubpart1 =  new TextButtonPartClass("Research", 150, tBackbitmap: ( this.OwnBitmap), bbx: num2, bby: 60, tred: (!flag), usefont: usefont, useshadow: flag, tMarcStyle: flag);
            this.mainx = this.AddSubPart( tsubpart1, num2, 60, 150, 35, 1);
          }
          let mut num3: i32 =  num2 + 160;
        }
      }
      str1: String = "Game Round is " + Strings.Trim(Conversion.Str( this.game.Data.Round));
      if (this.game.Data.AlternateRound > -1)
      {
        str2: String = "The date is ";
        DateTime dateTime = DateTime::new().AddYears(this.game.Data.StartData.Year - 1).AddMonths(this.game.Data.StartData.Month - 1).AddDays( (this.game.Data.StartData.Day - 1));
        if (this.game.Data.AlternateRound == 31)
        {
          dateTime = dateTime.AddMonths((this.game.Data.Round - 1) * 1);
        }
        else
        {
          TimeSpan timeSpan = new TimeSpan((this.game.Data.Round - 1) * this.game.Data.AlternateRound, 0, 0, 0);
          dateTime = dateTime.Add(timeSpan);
        }
        str1 = str2 + this.game.HandyFunctionsObj.GetMonth(dateTime.Month) + " " + Strings.Trim(Conversion.Str( dateTime.Day)) + " " + Strings.Trim(Conversion.Str( dateTime.Year));
      }
      SizeF sizeF1 = SizeF::new();
      if ( this.game.Data.RuleVar[839] == 1.0)
      {
        if (this.game.Data.Turn > -1)
        {
          let mut num4: i32 =  940;
          let mut num5: i32 =  16;
           let mut local1: &Graphics = &graphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCOPTSLOTS);
           let mut local2: &Bitmap = &bitmap;
          let mut x: i32 =  num4;
          let mut y: i32 =  num5;
          DrawMod.DrawSimple( local1,  local2, x, y);
          DrawMod.DrawTextColouredMarc( graphics, "PP", this.game.MarcFont8, num4 + 7, num5 + 4, Color.White);
          str3: String = this.game.Data.RegimeObj[this.game.Data.Turn].ResPts.ToString();
          SizeF sizeF2 = graphics.MeasureString(str3, this.game.MarcFont6);
          DrawMod.DrawTextColouredMarc( graphics, str3, this.game.MarcFont8,  Math.Round( ( (num4 + 57) - sizeF2.Width)), num5 + 2, Color.White);
          Rectangle trect = Rectangle::new( Math.Round( ( (num4 + 57) - sizeF2.Width)), num5 + 2, 75, 20);
          this.AddMouse( trect, "Political Points", "You need PP to play regime action cards");
        }
      }
      else
      {
        font: Font = Font::new(this.game.FontCol.Families[1], 13f, FontStyle.Regular, GraphicsUnit.Pixel);
        str4: String = "You have " + Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts)) + " pp. " + str1;
        SizeF sizeF3 = graphics.MeasureString(str4, font);
        DrawMod.DrawText( graphics, str4, font,  Math.Round( (870f - sizeF3.Width)), 20);
      }
      if (this.mainnr == 2)
        this.docardshand();
      if (this.mainnr == 3)
        this.docardsplayed();
      if (this.mainnr == 4)
        this.doreport();
      if (this.mainnr != 5)
        return;
      this.dostuff();
    }

     void clearsubstuff()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
      {
        this.RemoveSubPart(this.Text2Id);
        this.Text2Id = 0;
      }
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
      if (this.Text5id > 0)
        this.RemoveSubPart(this.Text5id);
      if (this.text6id > 0)
        this.RemoveSubPart(this.text6id);
      if (this.text7id > 0)
        this.RemoveSubPart(this.text7id);
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
      if (this.BAllyId > 0)
        this.RemoveSubPart(this.BAllyId);
      if (this.OptionsListId > 0)
      {
        this.RemoveSubPart(this.OptionsListId);
        this.OptionsListId = 0;
      }
      if (this.OptionsList2Id > 0)
      {
        this.RemoveSubPart(this.OptionsList2Id);
        this.OptionsList2Id = 0;
      }
      if (this.OptionsList3Id > 0)
      {
        this.RemoveSubPart(this.OptionsList3Id);
        this.OptionsList3Id = 0;
      }
      if (this.OptionsList4Id > 0)
      {
        this.RemoveSubPart(this.OptionsList4Id);
        this.OptionsList4Id = 0;
      }
      if (this.OptionsList5Id > 0)
      {
        this.RemoveSubPart(this.OptionsList5Id);
        this.OptionsList5Id = 0;
      }
      if (this.OptionsList6Id > 0)
      {
        this.RemoveSubPart(this.OptionsList6Id);
        this.OptionsList6Id = 0;
      }
      let mut index: i32 =  0;
      do
      {
        if (this.minicard[index] > 0)
          this.RemoveSubPart(this.minicard[index]);
        this.tempbmp[index] = (Bitmap) null;
        index += 1;
      }
      while (index <= 64);
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.game.EditObj.AreaSlot = -1;
      this.game.EditObj.AreaValue = -1;
      this.game.EditObj.AreaX = -1;
      this.game.EditObj.AreaY = -1;
      this.game.EditObj.AreaMap = -1;
    }

     void docardsplayed()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.BAllyId > 0)
        this.RemoveSubPart(this.BAllyId);
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
      if (this.Text5id > 0)
        this.RemoveSubPart(this.Text5id);
      if (this.text6id > 0)
        this.RemoveSubPart(this.text6id);
      if (this.text7id > 0)
        this.RemoveSubPart(this.text7id);
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
      let mut index1: i32 =  0;
      do
      {
        if (this.minicard[index1] > 0)
          this.RemoveSubPart(this.minicard[index1]);
        index1 += 1;
      }
      while (index1 <= 64);
      Graphics graphics1 = Graphics.FromImage((Image) this.OwnBitmap);
      graphics1.SmoothingMode = SmoothingMode.AntiAlias;
      graphics1.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics1.TextContrast = 1;
       let mut local1: &Graphics = &graphics1;
      bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.RESEARCHOVERPRINT);
       let mut local2: &Bitmap = &bitmap1;
      DrawMod.DrawSimple( local1,  local2, 610, 120);
      if (this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryCounter > 64)
      {
        this.OptionsList4Obj = ListClass::new();
        let mut tlistselect1: i32 =  -1;
        let mut num: i32 =  -1;
        let mut cardHistoryCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryCounter;
        for (let mut tdata: i32 =  0; tdata <= cardHistoryCounter; tdata += 1)
        {
          num += 1;
          if (this.detailnr == tdata)
            tlistselect1 = num;
          this.OptionsList4Obj.add(this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistory[tdata]].Title, tdata, Conversions.ToString(this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistory[tdata]].PPCost));
        }
        if (this.OptionsList4Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
        }
        else
        {
          ListClass optionsList4Obj = this.OptionsList4Obj;
          let mut tlistselect2: i32 =  tlistselect1;
          let mut game: GameClass = this.game;
           local3: Bitmap =  this.OwnBitmap;
          font: Font =  null;
           local4: Font =  font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList4Obj, 16, 550, tlistselect2, game, tHeader: "Action Cards", tShowPair: true, tValueWidth: 150, tbackbitmap: ( local3), bbx: 10, bby: 150, overruleFont: ( local4));
          this.OptionsList4Id = this.AddSubPart( tsubpart, 10, 160, 550, 304, 0);
        }
      }
      else
      {
        let mut cardHistoryCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryCounter;
        let mut num1: i32 =   Math.Round(Conversion.Int(  Math.Round(Conversion.Int(Math.Sqrt( (cardHistoryCounter + 1))) + 1.0) * 1.5));
        let mut num2: i32 =   Math.Round(Conversion.Int(520.0 /  num1));
        let mut num3: i32 =   Math.Round(Conversion.Int( num2 * 1.5));
        let mut num4: i32 =  -1;
        let mut num5: i32 =  0;
        let mut num6: i32 =  cardHistoryCounter;
        for (let mut index2: i32 =  0; index2 <= num6; index2 += 1)
        {
          num4 += 1;
          if (num4 >= num1)
          {
            num4 = 0;
            num5 += 1;
          }
          let mut x: i32 =   Math.Round(13.0 +  num4 * ( num2 * 1.1));
          let mut y: i32 =   Math.Round(156.0 +  num5 * ( num3 * 1.1));
          if (Information.IsNothing( this.tempbmp[index2]))
          {
            this.tempbmp[index2] = new Bitmap(num2, num3);
            this.tempbmp[index2].SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
            Graphics graphics2 = Graphics.FromImage((Image) this.tempbmp[index2]);
             let mut local5: &Graphics = &graphics2;
            bitmap2: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistory[index2], this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryRound[index2]);
             let mut local6: &Bitmap = &bitmap2;
            let mut w: i32 =  num2;
            let mut h: i32 =  num3;
            DrawMod.DrawScaled( local5,  local6, 0, 0, w, h);
          }
          int[] minicard = this.minicard;
          let mut index3: i32 =  index2;
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.tempbmp[index2]);
          let mut num7: i32 =  this.AddSubPart( tsubpart, x, y, num2, num3, 1);
          minicard[index3] = num7;
          if (this.detailnr == index2)
          {
            DrawMod.DrawRectangle( graphics1, x - 1, y - 1, num2 + 1, num3 + 1, 0, 0,  byte.MaxValue, 185);
            DrawMod.DrawRectangle( graphics1, x - 2, y - 2, num2 + 3, num3 + 3, 0, 0,  byte.MaxValue, 125);
            DrawMod.DrawRectangle( graphics1, x - 3, y - 3, num2 + 5, num3 + 5, 0, 0,  byte.MaxValue, 65);
          }
        }
      }
      if (this.detailnr <= -1)
        return;
       let mut local7: &Graphics = &graphics1;
      bitmap3: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistory[this.detailnr]);
       let mut local8: &Bitmap = &bitmap3;
      DrawMod.DrawSimple( local7,  local8, 660, 160);
      if (this.game.Data.AlternateRound > -1)
      {
        DateTime dateTime1 = DateTime::new().AddYears(this.game.Data.StartData.Year - 1).AddMonths(this.game.Data.StartData.Month - 1).AddDays( (this.game.Data.StartData.Day - 1));
        DateTime dateTime2;
        if (this.game.Data.AlternateRound == 31)
        {
          dateTime2 = dateTime1.AddMonths((this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryRound[this.detailnr] - 1) * 1);
        }
        else
        {
          TimeSpan timeSpan = new TimeSpan((this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryRound[this.detailnr] - 1) * this.game.Data.AlternateRound, 0, 0, 0);
          dateTime2 = dateTime1.Add(timeSpan);
        }
        str1: String;
        str2: String = str1 + this.game.HandyFunctionsObj.GetMonth(dateTime2.Month) + " " + Strings.Trim(Conversion.Str( dateTime2.Day)) + " " + Strings.Trim(Conversion.Str( dateTime2.Year));
        DrawMod.DrawTextColoured( graphics1, "Played " + str2, Font::new("Times New Roman", 19f, FontStyle.Bold, GraphicsUnit.Pixel), 715, 625, Color.White);
      }
      else
        DrawMod.DrawTextColoured( graphics1, "Played in round " + Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryRound[this.detailnr])), Font::new("Times New Roman", 19f, FontStyle.Bold, GraphicsUnit.Pixel), 715, 625, Color.White);
    }

     void docardshand()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
      if (this.Text5id > 0)
        this.RemoveSubPart(this.Text5id);
      if (this.BAllyId > 0)
        this.RemoveSubPart(this.BAllyId);
      if (this.text6id > 0)
        this.RemoveSubPart(this.text6id);
      if (this.text7id > 0)
        this.RemoveSubPart(this.text7id);
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
      let mut index1: i32 =  0;
      do
      {
        if (this.minicard[index1] > 0)
          this.RemoveSubPart(this.minicard[index1]);
        index1 += 1;
      }
      while (index1 <= 64);
      Graphics objGraphics = Graphics.FromImage((Image) this.OwnBitmap);
      objGraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objGraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objGraphics.TextContrast = 1;
       let mut local1: &Graphics = &objGraphics;
      bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.RESEARCHOVERPRINT);
       let mut local2: &Bitmap = &bitmap1;
      DrawMod.DrawSimple( local1,  local2, 610, 120);
      this.ClearMouse();
      SimpleList simpleList = SimpleList::new();
      actionCardCounter1: i32;
      if (this.game.Data.Turn > -1)
      {
        actionCardCounter1 = this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter;
        let mut num: i32 =  actionCardCounter1;
        for (index1 = 0; index1 <= num; index1 += 1)
        {
          let mut tweight: i32 =  this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index1]].ColorScheme * 1000 + this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index1];
          simpleList.Add(index1, tweight);
        }
      }
      simpleList.Sort();
      SubPartClass tsubpart1;
      bitmap2: Bitmap;
      Rectangle trect1;
      if ( this.game.Data.RuleVar[839] == 0.0)
      {
        if (this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter > 64)
        {
          this.OptionsList4Obj = ListClass::new();
          let mut tlistselect1: i32 =  -1;
          let mut num: i32 =  -1;
          let mut actionCardCounter2: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter;
          for (index1 = 0; index1 <= actionCardCounter2; index1 += 1)
          {
            num += 1;
            if (this.detailnr == index1)
              tlistselect1 = num;
            this.OptionsList4Obj.add(this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index1]].Title, index1, Conversions.ToString(this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index1]].PPCost));
          }
          if (this.OptionsList4Id > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, tlistselect1);
            this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
          }
          else
          {
            ListClass optionsList4Obj = this.OptionsList4Obj;
            let mut tlistselect2: i32 =  tlistselect1;
            let mut game: GameClass = this.game;
             local3: Bitmap =  this.OwnBitmap;
            font: Font =  null;
             local4: Font =  font;
            let mut tsubpart2: SubPartClass =  new ListSubPartClass(optionsList4Obj, 16, 550, tlistselect2, game, tHeader: "Action Cards", tShowPair: true, tValueWidth: 150, tbackbitmap: ( local3), bbx: 10, bby: 150, overruleFont: ( local4));
            this.OptionsList4Id = this.AddSubPart( tsubpart2, 10, 160, 550, 304, 0);
          }
        }
        else
        {
          let mut num1: i32 =   Math.Round(Conversion.Int(  Math.Round(Conversion.Int(Math.Sqrt( (actionCardCounter1 + 1))) + 1.0) * 1.5));
          let mut num2: i32 =   Math.Round(Conversion.Int(520.0 /  num1));
          let mut num3: i32 =   Math.Round(Conversion.Int( num2 * 1.5));
          let mut num4: i32 =  -1;
          let mut num5: i32 =  0;
          let mut counter: i32 =  simpleList.Counter;
          for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
          {
            index1 = simpleList.Id[index2];
            num4 += 1;
            if (num4 >= num1)
            {
              num4 = 0;
              num5 += 1;
            }
            let mut x: i32 =   Math.Round(13.0 +  num4 * ( num2 * 1.1));
            let mut y: i32 =   Math.Round(156.0 +  num5 * ( num3 * 1.1));
            if (Information.IsNothing( this.tempbmp[index1]))
            {
              this.tempbmp[index1] = new Bitmap(num2, num3);
              this.tempbmp[index1].SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
              Graphics graphics = Graphics.FromImage((Image) this.tempbmp[index1]);
               let mut local5: &Graphics = &graphics;
              bitmap3: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index1]);
               let mut local6: &Bitmap = &bitmap3;
              let mut w: i32 =  num2;
              let mut h: i32 =  num3;
              DrawMod.DrawScaled( local5,  local6, 0, 0, w, h);
            }
            int[] minicard = this.minicard;
            let mut index3: i32 =  index1;
            tsubpart1 =  ButtonPartClass::new(this.tempbmp[index1]);
            let mut num6: i32 =  this.AddSubPart( tsubpart1, x, y, num2, num3, 1);
            minicard[index3] = num6;
            if (this.detailnr == index1)
            {
              DrawMod.DrawRectangle( objGraphics, x - 1, y - 1, num2 + 1, num3 + 1, 0, 0,  byte.MaxValue, 185);
              DrawMod.DrawRectangle( objGraphics, x - 2, y - 2, num2 + 3, num3 + 3, 0, 0,  byte.MaxValue, 125);
              DrawMod.DrawRectangle( objGraphics, x - 3, y - 3, num2 + 5, num3 + 5, 0, 0,  byte.MaxValue, 65);
            }
          }
        }
      }
      else
      {
        let mut num: i32 =   Math.Round(Conversion.Int(1650.0 /  (simpleList.Counter + 1)));
        if (num > 110)
          num = 110;
        let mut x1: i32 =  25 - num;
        let mut y1: i32 =  140;
        let mut counter: i32 =  simpleList.Counter;
        for (index1 = 0; index1 <= counter; index1 += 1)
        {
          if (this.detailnr == -1)
            this.detailnr = simpleList.Id[index1];
          x1 += num;
          if (x1 > 550 & y1 > 200)
          {
            x1 = 25;
            y1 = 454;
          }
          else if (x1 > 550)
          {
            x1 = 25;
            y1 = 297;
          }
          let mut nr: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[simpleList.Id[index1]];
           let mut local7: &Graphics = &objGraphics;
          bitmap2 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, nr, size: 2);
           let mut local8: &Bitmap = &bitmap2;
          let mut x2: i32 =  x1;
          let mut y2: i32 =  y1;
          DrawMod.DrawSimple( local7,  local8, x2, y2);
          Rectangle trect2;
          if (this.game.Data.ActionCardObj[nr].MouseOver.Length > 0)
          {
            trect2 = Rectangle::new(x1, y1, 110, 147);
            trect1 = trect2;
            this.AddMouse( trect1, "REGIME CARD", this.game.Data.ActionCardObj[nr].MouseOver + "\r\nClick for close up and play option", simpleList.Id[index1] + 100);
          }
          else
          {
            trect1 = Rectangle::new(x1, y1, 110, 147);
            trect2 = trect1;
            this.AddMouse( trect2, "REGIME CARD", "Click for close up and play option", simpleList.Id[index1] + 100);
          }
        }
      }
      if (this.detailnr <= -1)
        return;
      usefont: Font;
      bool flag;
      if ( this.game.Data.RuleVar[839] == 0.0)
      {
        usefont =  null;
        flag = false;
      }
      else
      {
        usefont = this.game.MarcFont2;
        flag = true;
      }
      if ( this.game.Data.RuleVar[839] == 0.0)
      {
         let mut local9: &Graphics = &objGraphics;
        bitmap2 = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]);
         let mut local10: &Bitmap = &bitmap2;
        DrawMod.DrawSimple( local9,  local10, 660, 160);
      }
      else
      {
         let mut local11: &Graphics = &objGraphics;
        bitmap2 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]);
         let mut local12: &Bitmap = &bitmap2;
        DrawMod.DrawSimple( local11,  local12, 710, 260);
        if (this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]].MouseOver.Length > 0)
        {
          trect1 = Rectangle::new(710, 260, 190, 266);
          let mut trect3: &Rectangle = &trect1
          this.AddMouse( trect3, "SELECTED REGIME CARD", this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]].MouseOver + "\r\nClick on the 'play card' button to play this card.", simpleList.Id[index1] + 100);
        }
        else
        {
          trect1 = Rectangle::new(710, 260, 190, 266);
          let mut trect4: &Rectangle = &trect1
          this.AddMouse( trect4, "SELECTED REGIME CARD", "Click on the 'play card' button to play this card.", simpleList.Id[index1] + 100);
        }
      }
      if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >= this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]].PPCost | this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]].PPCost == 0)
      {
        if ( this.game.Data.RuleVar[839] == 0.0)
        {
          tsubpart1 =  new TextButtonPartClass("PLAY CARD", 200, tBackbitmap: ( this.OwnBitmap), bbx: 715, bby: 625, usefont: usefont, useshadow: flag, tMarcStyle: flag);
          this.B3Id = this.AddSubPart( tsubpart1, 715, 625, 200, 35, 1);
        }
        else
        {
          tsubpart1 =  new TextButtonPartClass("PLAY CARD", 200, "Click to play this card.",  this.OwnBitmap, 700, 560, usefont: usefont, useshadow: flag, tMarcStyle: flag);
          this.B3Id = this.AddSubPart( tsubpart1, 700, 560, 200, 35, 1);
        }
      }
      else if ( this.game.Data.RuleVar[839] == 0.0)
      {
        tsubpart1 =  new TextButtonPartClass("PLAY CARD", 200, tBackbitmap: ( this.OwnBitmap), bbx: 715, bby: 625, tinactive: true, usefont: usefont, useshadow: flag, tMarcStyle: flag);
        this.B3TextId = this.AddSubPart( tsubpart1, 715, 625, 200, 35, 1);
      }
      else
      {
        tsubpart1 =  new TextButtonPartClass("PLAY CARD", 200, "You do not have the PP to play this card.",  this.OwnBitmap, 700, 560, true, usefont: usefont, useshadow: flag, tMarcStyle: flag);
        this.B3TextId = this.AddSubPart( tsubpart1, 700, 560, 200, 35, 1);
      }
    }

     void doreport()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
      {
        this.RemoveSubPart(this.Text2Id);
        this.Text2Id = 0;
      }
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
      if (this.Text5id > 0)
        this.RemoveSubPart(this.Text5id);
      if (this.text6id > 0)
        this.RemoveSubPart(this.text6id);
      if (this.BAllyId > 0)
        this.RemoveSubPart(this.BAllyId);
      if (this.text7id > 0)
        this.RemoveSubPart(this.text7id);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      this.OptionsList5Obj = ListClass::new();
      let mut tlistselect1: i32 =  -1;
      let mut num1: i32 =  -1;
      let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
      for (let mut tdata: i32 =  0; tdata <= messCounter; tdata += 1)
      {
        if (this.game.Data.RegimeObj[this.game.Data.Turn].MessBackPic[tdata] == -2)
        {
          num1 += 1;
          if (this.detailnr == tdata)
            tlistselect1 = num1;
          let mut num2: i32 =  Strings.InStr(this.game.Data.RegimeObj[this.game.Data.Turn].MessString[tdata], "\r\n");
          str: String;
          if (Information.IsNothing( num2) | num2 <= 0)
          {
            str = Strings.Left(this.game.Data.RegimeObj[this.game.Data.Turn].MessString[tdata], 50) + "...";
          }
          else
          {
            str = Strings.Left(this.game.Data.RegimeObj[this.game.Data.Turn].MessString[tdata], num2);
            if (Strings.Len(str) > 50)
              str = Strings.Left(str, 50) + "...";
          }
          this.OptionsList5Obj.add(str, tdata);
        }
      }
      if (this.OptionsList5Obj.ListCount > -1)
      {
        if (this.OptionsList5Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList5Id)].Refresh(this.OptionsList5Obj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList5Id)] = true;
        }
        else
        {
          ListClass optionsList5Obj = this.OptionsList5Obj;
          let mut tlistselect2: i32 =  tlistselect1;
          let mut game: GameClass = this.game;
           local1: Bitmap =  this.OwnBitmap;
          font: Font =  null;
           local2: Font =  font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList5Obj, 16, 550, tlistselect2, game, tHeader: "This rounds reports", tbackbitmap: ( local1), bbx: 30, bby: 160, overruleFont: ( local2));
          this.OptionsList5Id = this.AddSubPart( tsubpart, 30, 160, 550, 304, 0);
        }
        if (this.detailnr <= -1)
          return;
        num3: i32;
        if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.detailnr] > -1)
        {
          let mut index: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.detailnr];
          let mut num4: i32 =  index < 10000 ? this.game.Data.EventPicNr[index] : this.game.Data.HistoricalUnitObj[index - 10000].CommanderSpriteID;
          if (num4 > -1)
          {
            let mut num5: i32 =  BitmapStore.GetWidth(num4);
            let mut num6: i32 =  BitmapStore.Getheight(num4);
            if (num5 > 320)
            {
              num6 =  Math.Round( num6 * (320.0 /  num5));
              num5 =  Math.Round( num5 * (320.0 /  num5));
            }
            let mut tsubpart: SubPartClass =  ButtonPartClass::new(num4, tResizeX: num5, tresizeY: num6);
            this.Text1Id = this.AddSubPart( tsubpart,  Math.Round(790.0 -  num5 / 2.0), 140, num5, num6, 0);
            DrawMod.DrawRectangle( graphics,  Math.Round(789.0 -  num5 / 2.0), 139, num5 + 2, num6 + 2, 0, 0, 0,  byte.MaxValue);
            DrawMod.DrawRectangle( graphics,  Math.Round(788.0 -  num5 / 2.0), 138, num5 + 4, num6 + 4,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
            num3 = 20 + num6 + 25;
          }
        }
        let mut trows: i32 =   Math.Round(35.0 -  num3 / 16.0);
        if (this.Text2Id != 0)
          return;
        let mut tsubpart1: SubPartClass =  new TextAreaClass(this.game, 360, trows, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), "", false, this.game.Data.RegimeObj[this.game.Data.Turn].MessString[this.detailnr], Color.White, tbackbitmap: ( this.OwnBitmap), bbx: 630, bby: (120 + num3));
        this.Text2Id = this.AddSubPart( tsubpart1, 630, 120 + num3, 360, 16 * (trows + 3), 0);
      }
      else
        DrawMod.DrawText( graphics, "No reports available.", this.game.GameFont1, 10, 150);
    }

     void dostuff()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
      if (this.Text5id > 0)
        this.RemoveSubPart(this.Text5id);
      if (this.text6id > 0)
        this.RemoveSubPart(this.text6id);
      if (this.BAllyId > 0)
        this.RemoveSubPart(this.BAllyId);
      if (this.text7id > 0)
        this.RemoveSubPart(this.text7id);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
       let mut local1: &Graphics = &graphics;
      bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.RESEARCHOVERPRINT);
       let mut local2: &Bitmap = &bitmap1;
      DrawMod.DrawSimple( local1,  local2, 610, 120);
      this.OptionsListObj = ListClass::new();
      if (this.detailnr > this.game.Data.ResearchCounter)
        this.detailnr = -1;
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      if (this.game.Data.ResearchCounter > -1)
      {
        let mut researchCounter: i32 =  this.game.Data.ResearchCounter;
        for (let mut tdata: i32 =  0; tdata <= researchCounter; tdata += 1)
        {
          if (!this.game.Data.RegimeObj[this.pregnr].ResField[tdata])
          {
            let mut num3: i32 =  1;
            if (this.game.Data.ResearchObj[tdata].PreReq > -1 && !this.game.Data.RegimeObj[this.pregnr].ResField[this.game.Data.ResearchObj[tdata].PreReq])
              num3 = 0;
            if (this.game.Data.ResearchObj[tdata].PreReq2 > -1 && !this.game.Data.RegimeObj[this.pregnr].ResField[this.game.Data.ResearchObj[tdata].PreReq2])
              num3 = 0;
            if (this.game.Data.ResearchObj[tdata].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] < 0 | this.game.Data.ResearchObj[tdata].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] > 9998)
              num3 = 0;
            if (num3 == 1)
            {
              num2 += 1;
              if (this.detailnr == tdata)
                num1 = num2;
              tname: String = this.game.Data.ResearchObj[tdata].Name;
              let mut Number: i32 =  0;
              if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, true) &  this.game.Data.RuleVar[530] == 1.0)
              {
                let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                for (let mut reg2: i32 =  0; reg2 <= regimeCounter; reg2 += 1)
                {
                  if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, reg2) & this.game.Data.Turn != reg2 && this.game.Data.RegimeObj[reg2].ResField[tdata])
                    Number += 1;
                }
              }
              if (Number > 0)
                tname = tname + " (" + Strings.Trim(Conversion.Str( Number)) + ")";
              this.OptionsListObj.add(tname, tdata);
            }
          }
        }
        this.OptionsListObj.Sort();
        let mut tlistselect1: i32 =  -1;
        let mut listCount: i32 =  this.OptionsListObj.ListCount;
        for (let mut index: i32 =  0; index <= listCount; index += 1)
        {
          if (this.detailnr == this.OptionsListObj.ListData[index])
            tlistselect1 = index;
        }
        if (this.OptionsListId > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
        }
        else
        {
          ListClass optionsListObj = this.OptionsListObj;
          let mut tlistselect2: i32 =  tlistselect1;
          let mut game: GameClass = this.game;
           local3: Bitmap =  this.OwnBitmap;
          font: Font =  null;
           local4: Font =  font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsListObj, 30, 150, tlistselect2, game, tHeader: "Available Research", tbackbitmap: ( local3), bbx: 400, bby: 160, overruleFont: ( local4));
          this.OptionsListId = this.AddSubPart( tsubpart, 400, 160, 150, 528, 0);
        }
      }
      this.OptionsList3Obj = ListClass::new();
      if (this.detailnr3 > this.game.Data.ResearchCounter)
        this.detailnr3 = -1;
      num1 = -1;
      let mut num4: i32 =  -1;
      if (this.game.Data.ResearchCounter > -1)
      {
        let mut researchCounter: i32 =  this.game.Data.ResearchCounter;
        for (let mut tdata: i32 =  0; tdata <= researchCounter; tdata += 1)
        {
          if (!this.game.Data.RegimeObj[this.pregnr].ResField[tdata])
          {
            let mut num5: i32 =  1;
            if (this.game.Data.ResearchObj[tdata].PreReq > -1 && !this.game.Data.RegimeObj[this.pregnr].ResField[this.game.Data.ResearchObj[tdata].PreReq])
              num5 = 0;
            if (this.game.Data.ResearchObj[tdata].PreReq2 > -1 && !this.game.Data.RegimeObj[this.pregnr].ResField[this.game.Data.ResearchObj[tdata].PreReq2])
              num5 = 0;
            if (this.game.Data.ResearchObj[tdata].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] < 0 | this.game.Data.ResearchObj[tdata].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] > 9998)
              num5 = 1;
            if (num5 == 0)
            {
              num4 += 1;
              if (this.detailnr3 == tdata)
                num1 = num4;
              tname: String = this.game.Data.ResearchObj[tdata].Name;
              let mut Number: i32 =  0;
              if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, true) &  this.game.Data.RuleVar[530] == 1.0)
              {
                let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                for (let mut reg2: i32 =  0; reg2 <= regimeCounter; reg2 += 1)
                {
                  if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, reg2) & this.game.Data.Turn != reg2 && this.game.Data.RegimeObj[reg2].ResField[tdata])
                    Number += 1;
                }
              }
              if (Number > 0)
                tname = tname + " (" + Strings.Trim(Conversion.Str( Number)) + ")";
              this.OptionsList3Obj.add(tname, tdata);
            }
          }
        }
        this.OptionsList3Obj.Sort();
        let mut tlistselect3: i32 =  -1;
        let mut listCount: i32 =  this.OptionsList3Obj.ListCount;
        for (let mut index: i32 =  0; index <= listCount; index += 1)
        {
          if (this.detailnr3 == this.OptionsList3Obj.ListData[index])
            tlistselect3 = index;
        }
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, tlistselect3);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          ListClass optionsList3Obj = this.OptionsList3Obj;
          let mut tlistselect4: i32 =  tlistselect3;
          let mut game: GameClass = this.game;
           local5: Bitmap =  this.OwnBitmap;
          font: Font =  null;
           local6: Font =  font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList3Obj, 30, 150, tlistselect4, game, tHeader: "Not yet available", tbackbitmap: ( local5), bbx: 220, bby: 160, overruleFont: ( local6));
          this.OptionsList3Id = this.AddSubPart( tsubpart, 220, 160, 150, 528, 0);
        }
      }
      num1 = -1;
      let mut num6: i32 =  -1;
      if (this.detailnr2 > this.game.Data.ResearchCounter)
        this.detailnr2 = -1;
      this.OptionsList2Obj = ListClass::new();
      if (this.game.Data.ResearchCounter > -1)
      {
        let mut researchCounter1: i32 =  this.game.Data.ResearchCounter;
        for (let mut tdata: i32 =  0; tdata <= researchCounter1; tdata += 1)
        {
          if (this.game.Data.RegimeObj[this.pregnr].ResField[tdata])
          {
            let mut num7: i32 =  1;
            let mut researchCounter2: i32 =  this.game.Data.ResearchCounter;
            for (let mut index: i32 =  0; index <= researchCounter2; index += 1)
            {
              if (this.game.Data.RegimeObj[this.pregnr].ResField[index] && this.game.Data.ResearchObj[index].Blocks == tdata)
              {
                num7 = 0;
                if (this.game.HandyFunctionsObj.HasAllies(this.pregnr, true) &  this.game.Data.RuleVar[530] == 1.0)
                  num7 = 1;
              }
            }
            if (num7 == 1)
            {
              num6 += 1;
              if (this.detailnr2 == tdata)
                num1 = num6;
              tname: String = this.game.Data.ResearchObj[tdata].Name;
              let mut Number: i32 =  0;
              if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, true) &  this.game.Data.RuleVar[530] == 1.0)
              {
                let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                for (let mut reg2: i32 =  0; reg2 <= regimeCounter; reg2 += 1)
                {
                  if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, reg2) & this.game.Data.Turn != reg2 && this.game.Data.RegimeObj[reg2].ResField[tdata])
                    Number += 1;
                }
              }
              if (Number > 0)
                tname = tname + " (" + Strings.Trim(Conversion.Str( Number)) + ")";
              this.OptionsList2Obj.add(tname, tdata);
            }
          }
        }
      }
      this.OptionsList2Obj.Sort();
      let mut tlistselect5: i32 =  -1;
      let mut listCount1: i32 =  this.OptionsList2Obj.ListCount;
      for (let mut index: i32 =  0; index <= listCount1; index += 1)
      {
        if (this.detailnr2 == this.OptionsList2Obj.ListData[index])
          tlistselect5 = index;
      }
      if (this.OptionsList2Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, tlistselect5);
        this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
      }
      else
      {
        ListClass optionsList2Obj = this.OptionsList2Obj;
        let mut tlistselect6: i32 =  tlistselect5;
        let mut game: GameClass = this.game;
         local7: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local8: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList2Obj, 30, 150, tlistselect6, game, tHeader: "Known Research", tHeaderCenter: false, tbackbitmap: ( local7), bbx: 40, bby: 160, overruleFont: ( local8));
        this.OptionsList2Id = this.AddSubPart( tsubpart, 40, 160, 150, 528, 0);
      }
      let mut index1: i32 =  this.detailnr;
      if (index1 == -1)
        index1 = this.detailnr2;
      if (index1 == -1)
        index1 = this.detailnr3;
      if (index1 <= -1)
        return;
      let mut x1: i32 =  650;
      let mut num8: i32 =  0;
      if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, true))
      {
        let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
        for (let mut reg2: i32 =  0; reg2 <= regimeCounter; reg2 += 1)
        {
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, reg2) & this.game.Data.Turn != reg2 &  this.game.Data.RuleVar[530] == 1.0 && this.game.Data.RegimeObj[reg2].ResField[index1])
          {
            if (num8 == 0)
              DrawMod.DrawText( graphics, "Allies:", this.game.GameFont1, x1, 145);
            num8 = 1;
            x1 += 37;
             let mut local9: &Graphics = &graphics;
            bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.Data.RegimeObj[reg2].HQSpriteNr);
             let mut local10: &Bitmap = &bitmap2;
            let mut x2: i32 =  x1;
            DrawMod.DrawSimple( local9,  local10, x2, 143);
          }
        }
      }
      if (index1 == this.detailnr)
        DrawMod.DrawSimple( graphics,  this.game.CARD3B, 648, 173);
      else if (index1 == this.detailnr2)
        DrawMod.DrawSimple( graphics,  this.game.CARD2B, 648, 173);
      else if (index1 == this.detailnr3)
        DrawMod.DrawSimple( graphics,  this.game.CARD1B, 648, 173);
      DrawMod.DrawTextColoured( graphics, this.game.Data.ResearchObj[index1].Name, Font::new("Arial Black", 19f, FontStyle.Regular, GraphicsUnit.Pixel), 675, 201, Color.White);
      if (this.game.Data.ResearchObj[index1].SFTypePic > -1)
      {
        let mut picSpriteId: i32 =  this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].PicSpriteID;
        if (this.game.Data.RegimeObj[this.game.Data.Turn].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].ExtraCounter;
          for (let mut index2: i32 =  0; index2 <= extraCounter; index2 += 1)
          {
            if (this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].ExtraCode[index2] == this.game.Data.RegimeObj[this.game.Data.Turn].ExtraGraphicUse)
              picSpriteId = this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].ExtraPicSpriteID[index2];
          }
        }
         let mut local11: &Graphics = &graphics;
        bitmap3: Bitmap = BitmapStore.GetBitmap(picSpriteId);
         let mut local12: &Bitmap = &bitmap3;
        DrawMod.DrawScaled( local11,  local12, 665, 240, 260, 194);
         let mut local13: &Graphics = &graphics;
        bitmap4: Bitmap = BitmapStore.GetBitmap(this.game.ACTIONFRAME);
         let mut local14: &Bitmap = &bitmap4;
        DrawMod.DrawSimple( local13,  local14, 665, 240);
      }
      if (this.game.Data.ResearchObj[index1].PreReq > -1)
      {
        tstring: String = "PreReq: " + this.game.Data.ResearchObj[this.game.Data.ResearchObj[index1].PreReq].Name;
        DrawMod.DrawTextColoured( graphics, tstring, Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 675, 450, Color.Black);
      }
      if (this.game.Data.ResearchObj[index1].PreReq2 > -1)
      {
        tstring: String = "PreReq: " + this.game.Data.ResearchObj[this.game.Data.ResearchObj[index1].PreReq2].Name;
        DrawMod.DrawTextColoured( graphics, tstring, Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 675, 465, Color.Black);
      }
      tstring1: String = this.game.Data.ResearchObj[index1].CostType <= -1 ? Conversion.Str( Conversion.Int( this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] * this.game.Data.ResCostMod)) + "pp" : Conversion.Str( Conversion.Int( this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] * this.game.Data.ResCostMod)) + " " + this.game.Data.RegimeSlotName[this.game.Data.ResearchObj[index1].CostType];
      DrawMod.DrawTextColoured( graphics, tstring1, Font::new("Times New Roman", 25f, FontStyle.Regular, GraphicsUnit.Pixel), 670, 480, Color.Black);
      if (this.game.Data.ResearchObj[index1].CostType == -1)
      {
        if (this.detailnr2 == -1 & this.detailnr3 == -1 &  Conversion.Int(this.game.Data.ResCostMod *  this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup]) <=  this.game.Data.RegimeObj[this.regnr].ResPts)
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Buy", 100, "Buy selected researchfield",  this.OwnBitmap, 750, 635);
          this.B2Id = this.AddSubPart( tsubpart, 750, 635, 100, 35, 1);
        }
      }
      else if (this.detailnr2 == -1 & this.detailnr3 == -1 &  Conversion.Int(this.game.Data.ResCostMod *  this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup]) <=  this.game.Data.RegimeObj[this.regnr].RegimeSlot[this.game.Data.ResearchObj[index1].CostType])
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Buy", 100, "Buy selected researchfield",  this.OwnBitmap, 750, 635);
        this.B2Id = this.AddSubPart( tsubpart, 750, 635, 100, 35, 1);
      }
      let mut num9: i32 =  0;
      if (this.game.HandyFunctionsObj.HasAllies(this.game.Data.Turn, true) & this.game.Data.RegimeObj[this.game.Data.Turn].ResField[index1] &  this.game.Data.RuleVar[530] == 1.0 && this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] > -1)
      {
        let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
        for (let mut reg2: i32 =  0; reg2 <= regimeCounter; reg2 += 1)
        {
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, reg2) & this.game.Data.Turn != reg2 && !this.game.Data.RegimeObj[reg2].ResField[index1])
            num9 += 1;
        }
        let mut preReq: i32 =  this.game.Data.ResearchObj[index1].PreReq;
        if (preReq > -1 && this.game.Data.ResearchObj[preReq].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] == -1)
          num9 = 0;
        let mut preReq2: i32 =  this.game.Data.ResearchObj[index1].PreReq2;
        if (preReq2 > -1 && this.game.Data.ResearchObj[preReq2].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] == -1)
          num9 = 0;
        if (num9 > 0)
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Give to Ally", 200, "Give selected researchfield",  this.OwnBitmap, 700, 635);
          this.BAllyId = this.AddSubPart( tsubpart, 700, 635, 200, 35, 1);
        }
      }
      let mut tsubpart1: SubPartClass =  new TextAreaClass(this.game, 280, 4, Font::new(this.game.FontCol.Families[1], 13f, FontStyle.Regular, GraphicsUnit.Pixel), "", false, this.game.Data.ResearchObj[index1].Text, Color.Black, tbackbitmap: ( this.OwnBitmap), bbx: 665, bby: 510, tHideShade: true, tBlockScroller: true);
      this.Text4id = this.AddSubPart( tsubpart1, 665, 510, 280, 80, 0);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if ((nr == 27 | nr == 32) & this.game.Data.CampaignRoom == -1)
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.EditObj.OrderType = 0;
          windowReturnClass.AddCommand(3, 3);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleActionCard: WindowReturnClass(t2: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.game.Data.ActionCardObj[t2].AreaSlot > -1)
      {
        if (this.game.EditObj.AreaX == -1)
        {
          this.game.ProcessingObj.PlayCardPreEvent(this.game.Data.Turn, this.detailnr);
          this.game.EditObj.DoCardSlot = this.detailnr;
          this.game.EditObj.HandCard = -1;
          this.clearsubstuff();
          this.game.EditObj.AreaSlot = this.game.Data.ActionCardObj[t2].AreaSlot;
          this.game.EditObj.AreaValue = this.game.Data.ActionCardObj[t2].AreaValue;
          this.game.EditObj.PopupValue = 1;
          windowReturnClass.AddCommand(5, 10);
          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        let mut num: i32 =   Interaction.MsgBox( "Error. Cant have selected an Area X,Y already.");
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (this.game.Data.ActionCardObj[t2].UnitSelect)
      {
        this.game.ProcessingObj.PlayCardPreEvent(this.game.Data.Turn, this.detailnr);
        this.game.EditObj.DoCardSlot = this.detailnr;
        this.game.EditObj.HandCard = -1;
        this.clearsubstuff();
        this.game.EditObj.PopupValue = 3;
        windowReturnClass.AddCommand(5, 10);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
      this.game.ProcessingObj.PlayCard(this.game.Data.Turn, this.detailnr);
      if (this.game.EditObj.DoQuit)
      {
        this.game.Data = DataClass::new();
        this.game.EditObj.DoQuit = false;
        this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
        if (this.game.Data.UseAI == 1)
          this.game.NewAIObj.LastRegime = -1;
        this.game.EditObj.ShowInitialMenu = true;
        windowReturnClass.AddCommand(3, 1);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (Strings.Len(this.game.Data.LoadGame) > 0 & this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter <= messCounter)
      {
        this.game.FormRef.Cursor = Cursors.WaitCursor;
        Form formRef =  this.game.FormRef;
        this.game.HandyFunctionsObj.LoadGameNow();
        this.game.FormRef = (Form1) formRef;
        this.game.FormRef.Cursor = Cursors.Default;
        windowReturnClass.AddCommand(3, 4);
        return windowReturnClass;
      }
      t2 = 0;
      let mut locCounter: i32 =  this.game.Data.LocCounter;
      for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
      {
        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
        {
          let mut index: i32 =  0;
          do
          {
            if (this.game.Data.LocObj[locnr].Production[index] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index]).result)
            {
              t2 += 1;
              this.game.Data.LocObj[locnr].Production[index] = -1;
              this.game.Data.LocObj[locnr].ProdPointRemainder[index] = 0;
              this.game.Data.LocObj[locnr].ProdPercent[index] = 0;
            }
            index += 1;
          }
          while (index <= 3);
        }
      }
      if (t2 > 0)
      {
        let mut num1: i32 =   Interaction.MsgBox( (Conversion.Str( t2) + " production lines have been cancelled due to this action card being played."), Title: ( "Shadow Empire : Planetary Conquest"));
      }
      if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
      {
        this.clearsubstuff();
        this.game.EditObj.PopupValue = 0;
        this.game.EditObj.FromMessage = messCounter + 1;
        windowReturnClass.AddCommand(5, 10);
        this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      this.detailnr = -1;
      this.domain();
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      for (let mut mouseCounter: i32 =  this.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (this.MouseData[mouseCounter] > 0 && x > this.MouseRect[mouseCounter].X & x < this.MouseRect[mouseCounter].X + this.MouseRect[mouseCounter].Width && y > this.MouseRect[mouseCounter].Y & y < this.MouseRect[mouseCounter].Y + this.MouseRect[mouseCounter].Height && this.MouseData[mouseCounter] >= 100 & this.MouseData[mouseCounter] <= 100 + this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter + 100)
        {
          this.detailnr = this.MouseData[mouseCounter] - 100;
          this.detailnr2 = -1;
          this.detailnr3 = -1;
          this.domain();
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        }
      }
      if (this.SubPartCounter > -1)
      {
        if (this.dodetailnr > -1)
        {
          windowReturnClass2: WindowReturnClass;
          return windowReturnClass2;
        }
        let mut subPartCounter: i32 =  this.SubPartCounter;
label_71:
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.main2)
            {
              this.mainnr = 2;
              this.clearsubstuff();
              this.domain();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.main3)
            {
              this.mainnr = 3;
              this.clearsubstuff();
              this.domain();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.main4)
            {
              this.mainnr = 4;
              this.clearsubstuff();
              this.domain();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.main5)
            {
              this.mainnr = 5;
              this.clearsubstuff();
              this.domain();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.OptionsList4Id)
            {
              let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.detailnr2 = -1;
                this.detailnr3 = -1;
                if (this.mainnr == 2)
                  this.docardshand();
                if (this.mainnr == 3)
                  this.docardsplayed();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.B3Id)
              return this.HandleActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]);
            if (num1 == this.Text2Id)
            {
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.OptionsList5Id)
            {
              let mut num3: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                this.detailnr = num3;
                this.detailnr2 = -1;
                this.detailnr3 = -1;
                if (this.Text2Id > 0)
                {
                  this.RemoveSubPart(this.Text2Id);
                  this.Text2Id = 0;
                }
                this.domain();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.but1id)
            {
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.OrderType = 0;
              this.game.EditObj.AreaX = -1;
              this.game.EditObj.AreaY = -1;
              this.game.EditObj.AreaSlot = -1;
              this.game.EditObj.AreaValue = -1;
              windowReturnClass1.AddCommand(3, 11);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.B2Id)
            {
              this.game.ProcessingObj.BuyResearch(this.pplnr, this.regnr, this.detailnr);
              SimpleList simpleList = SimpleList::new();
              let mut itemTypeCounter: i32 =  this.game.Data.ItemTypeCounter;
              Number: i32;
              for (let mut itemtypenr: i32 =  0; itemtypenr <= itemTypeCounter; itemtypenr += 1)
              {
                if (this.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[0] == this.detailnr | this.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[1] == this.detailnr | this.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[2] == this.detailnr | this.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[3] == this.detailnr | this.game.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[4] == this.detailnr && this.game.Data.ItemTypeObj[itemtypenr].Blocks > -1)
                {
                  let mut blocks: i32 =  this.game.Data.ItemTypeObj[itemtypenr].Blocks;
                  let mut locCounter: i32 =  this.game.Data.LocCounter;
                  for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
                  {
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
                    {
                      let mut index2: i32 =  0;
                      do
                      {
                        if (this.game.Data.LocObj[locnr].Production[index2] == this.game.Data.ItemTypeObj[itemtypenr].Blocks && this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, itemtypenr).result)
                        {
                          this.game.Data.LocObj[locnr].Production[index2] = itemtypenr;
                          Number += 1;
                        }
                        index2 += 1;
                      }
                      while (index2 <= 3);
                    }
                  }
                }
              }
              if (Number > 0)
              {
                let mut num4: i32 =   Interaction.MsgBox( ("Automatically switched " + Conversion.Str( Number) + " production line(s)."), Title: ( "Shadow Empire : Planetary Conquest"));
              }
              this.detailnr = -1;
              this.detailnr2 = -1;
              this.detailnr3 = -1;
              this.domain();
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.OptionsListId)
            {
              let mut num5: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num5 > -1)
              {
                this.detailnr = num5;
                this.detailnr2 = -1;
                this.detailnr3 = -1;
                this.domain();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.BAllyId)
            {
              Form3::new( this.formref).Initialize(this.game.Data, 54, this.detailnr2);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.OptionsList2Id)
            {
              let mut num6: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num6 > -1)
              {
                this.detailnr2 = num6;
                this.detailnr = -1;
                this.detailnr3 = -1;
                this.domain();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == this.OptionsList3Id)
            {
              let mut num7: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num7 > -1)
              {
                this.detailnr3 = num7;
                this.detailnr = -1;
                this.detailnr2 = -1;
                this.domain();
              }
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            let mut index3: i32 =  0;
            while (this.SubPartID[index1] != this.minicard[index3])
            {
              index3 += 1;
              if (index3 > 64)
                goto label_71;
            }
            this.detailnr = index3;
            this.domain();
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub fn DoRefresh() => this.domain();

    pub fn PopUpRefresh()
    {
      this.game.EditObj.AreaSlot = -1;
      this.game.EditObj.AreaX = -1;
      this.game.EditObj.AreaY = -1;
      this.game.EditObj.AreaValue = -1;
      this.game.EditObj.DoCardSlot = -1;
      this.mainnr = 2;
      this.domain();
    }
  }
}
