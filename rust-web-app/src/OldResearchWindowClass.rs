// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.OldResearchWindowClass
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
  pub class OldResearchWindowClass : WindowClass
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
     ATListClass OptionsListObj;
     OptionsList2Id: i32;
     ATListClass OptionsList2Obj;
     OptionsList3Id: i32;
     ATListClass OptionsList3Obj;
     OptionsList4Id: i32;
     ATListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ATListClass OptionsList5Obj;
     OptionsList6Id: i32;
     ATListClass OptionsList6Obj;
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
     lasttab: i32;

    pub OldResearchWindowClass( tGame: GameClass, tempInt: i32)
      : base( tGame, 1024, 768, 7)
    {
      this.minicard = new int[65];
      this.tempbmp = new Bitmap[65];
      this.game.EditObj.DoCardSlot = -1;
      this.game.EditObj.HandCard = -1;
      this.remainderofnew();
    }

    pub OldResearchWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
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
      if (this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter > -1 &  this.game.Data.RuleVar[502] < 1.0)
        this.mainnr = 2;
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
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.Text5id > 0)
        this.RemoveSubPart(this.Text5id);
      if (this.game.EditObj.DoCardSlot > -1)
        return;
      if (this.game.Data.CampaignRoom > -1)
      {
        if ( this.game.Data.RuleVar[839] == 0.0)
        {
          if (this.game.EditObj.CampaignRoomBitmap > -1)
            this.NewBackGroundAndClearAll(1024, 768, this.game.Data.EventPicNr[this.game.EditObj.CampaignRoomBitmap]);
          else
            this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND2MARC);
        }
        else
          this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND1MARC);
      }
      else
        this.NewBackGroundAndClearAll(1024, 768, this.game.BACKGROUND2MARC);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      Expression.SmoothingMode = SmoothingMode.AntiAlias;
      Expression.TextRenderingHint = TextRenderingHint.AntiAlias;
      Expression.TextContrast = 1;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (this.game.Data.CampaignRoom > -1 & Strings.Len(this.game.EditObj.CampaignRoomTitle) > 0)
      {
        if ( this.game.Data.RuleVar[839] != 1.0)
        {
           let mut local: &Graphics = &Expression;
          let mut rect1: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(25, 29, 625, 27);
          let mut rect2: &Rectangle = &rectangle2
          campaignRoomTitle: String = this.game.EditObj.CampaignRoomTitle;
          DrawMod.MakeFullBoxVic2( local, rect1, "", rect2, campaignRoomTitle);
        }
      }
      else
      {
         let mut local: &Graphics = &Expression;
        let mut rect1: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(25, 29, 625, 27);
        let mut rect2: &Rectangle = &rectangle2
        DrawMod.MakeFullBoxVic2( local, rect1, "", rect2, "Decision Room");
      }
      if (this.game.Data.CampaignRoom == -1)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONQUIT);
        this.but1id = this.AddSubPart( tsubpart, 952, 22, 32, 32, 1);
      }
      let mut num1: i32 =  25;
      if (Strings.Len(this.game.EditObj.CampaignRoomTitle) > 0)
        num1 = 25;
      font: Font;
      bool flag;
      if ( this.game.Data.RuleVar[839] == 1.0)
      {
        font = this.game.MarcFont4;
        flag = true;
      }
      else
      {
        font = this.game.VicFont2;
        flag = false;
      }
      num2: i32;
      if ( this.game.Data.RuleVar[502] == 0.0 & this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter > -1)
      {
        buttontext: String = "Cards".to_owned();
        if (this.mainnr != 2)
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass(buttontext, 150, "Click to see your cards",  this.OwnBitmap, num1, 60);
          this.main2 = this.AddSubPart( tsubpart, num1, 60, 150, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass(buttontext, 150, "Your viewing your cards already.",  this.OwnBitmap, num1, 60, tred: true);
          this.mainx = this.AddSubPart( tsubpart, num1, 60, 150, 35, 1);
        }
        let mut num3: i32 =  num1 + 160;
        if ( this.game.Data.RuleVar[510] == 0.0 & this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryCounter > -1)
        {
          if (this.mainnr != 3)
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("Active Cards", 150, tBackbitmap: ( this.OwnBitmap), bbx: num3, bby: 60);
            this.main3 = this.AddSubPart( tsubpart, num3, 60, 150, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new TextButtonPartClass("Active Cards", 150, tBackbitmap: ( this.OwnBitmap), bbx: num3, bby: 60, tred: true);
            this.mainx = this.AddSubPart( tsubpart, num3, 60, 150, 35, 1);
          }
          num2 = num3 + 160;
        }
        else
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Active Cards", 150, "Active cards not used",  this.OwnBitmap, num3, 60, true);
          this.Text5id = this.AddSubPart( tsubpart, num3, 60, 150, 35, 1);
          num2 = num3 + 160;
        }
      }
      else
      {
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Cards", 150, "Cards not active in scenario",  this.OwnBitmap, num1, 60, true);
        this.Text3Id = this.AddSubPart( tsubpart1, num1, 60, 150, 35, 1);
        let mut num4: i32 =  num1 + 160;
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Active Cards", 150, "Active cards not used",  this.OwnBitmap, num4, 60, true);
        this.Text5id = this.AddSubPart( tsubpart2, num4, 60, 150, 35, 1);
        num2 = num4 + 160;
      }
      buttontext1: String = "Reports".to_owned();
      if ( this.game.Data.RuleVar[839] == 1.0)
        buttontext1 = Strings.UCase(buttontext1);
      if (this.mainnr != 4)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(buttontext1, 150, "Click to view your reports.",  this.OwnBitmap, num2, 60);
        this.main4 = this.AddSubPart( tsubpart, num2, 60, 150, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(buttontext1, 150, "Your currently viewing your reports already.",  this.OwnBitmap, num2, 60, tred: true);
        this.mainx = this.AddSubPart( tsubpart, num2, 60, 150, 35, 1);
      }
      let mut num5: i32 =  num2 + 160;
      if ( this.game.Data.RuleVar[501] < 1.0)
      {
        if (this.mainnr != 5)
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Research", 150, tBackbitmap: ( this.OwnBitmap), bbx: num5, bby: 60);
          this.main5 = this.AddSubPart( tsubpart, num5, 60, 150, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("Research", 150, tBackbitmap: ( this.OwnBitmap), bbx: num5, bby: 60, tred: true);
          this.mainx = this.AddSubPart( tsubpart, num5, 60, 150, 35, 1);
        }
        let mut num6: i32 =  num5 + 160;
      }
      str1: String = Strings.Trim(Conversion.Str( this.game.Data.Round));
      if (this.game.Data.AlternateRound > -1)
      {
        str2: String = "";
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
       let mut local1: &Graphics = &Expression;
      rectangle2 = Rectangle::new(675, 15, 150, 14);
      let mut rect1_1: &Rectangle = &rectangle2
      Rectangle rectangle3 = Rectangle::new(675, 29, 150, 27);
      let mut rect2_1: &Rectangle = &rectangle3
      txt2_1: String = str1;
      DrawMod.MakeFullBoxVic2( local1, rect1_1, "DATE/ROUND", rect2_1, txt2_1);
      str3: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts));
       let mut local2: &Graphics = &Expression;
      rectangle3 = Rectangle::new(850, 15, 75, 14);
      let mut rect1_2: &Rectangle = &rectangle3
      rectangle2 = Rectangle::new(850, 29, 75, 27);
      let mut rect2_2: &Rectangle = &rectangle2
      txt2_2: String = str3;
      DrawMod.MakeFullBoxVic2( local2, rect1_2, "PP", rect2_2, txt2_2);
      if (this.mainnr == 2)
        this.docardshand();
      if (this.mainnr == 3)
        this.docardsplayed();
      if (this.mainnr == 4)
        this.doreport();
      if (this.mainnr == 5)
        this.dostuff();
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
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
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
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
      if (this.BAllyId > 0)
        this.RemoveSubPart(this.BAllyId);
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
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
      Graphics Expression1 = Graphics.FromImage((Image) this.OwnBitmap);
      Expression1.SmoothingMode = SmoothingMode.AntiAlias;
      Expression1.TextRenderingHint = TextRenderingHint.AntiAlias;
      Expression1.TextContrast = 1;
      DrawMod.DrawBlock( Expression1, 610, 120, 377, 576,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  Expression1, 610, 120, 377, 576, -1, -1);
      Graphics Expression2;
      if (this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryCounter > 64)
      {
        this.OptionsList4Obj = ATListClass::new();
        let mut tlistselect: i32 =  -1;
        let mut num: i32 =  -1;
        let mut cardHistoryCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryCounter;
        for (let mut tdata: i32 =  0; tdata <= cardHistoryCounter; tdata += 1)
        {
          num += 1;
          if (this.detailnr == tdata)
            tlistselect = num;
          this.OptionsList4Obj.add(this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistory[tdata]].Title, tdata, Conversions.ToString(this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistory[tdata]].PPCost));
        }
        if (this.OptionsList4Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, tlistselect);
          this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
        }
        else
        {
          let mut tsubpart: SubPartClass =  new ATListSubPartClass(this.OptionsList4Obj, 16, 550, tlistselect, this.game, tHeader: "Action Cards", tShowPair: true, tValueWidth: 150, tbackbitmap: ( this.OwnBitmap), bbx: 10, bby: 150);
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
            Graphics graphics = Graphics.FromImage((Image) this.tempbmp[index2]);
             let mut local1: &Graphics = &graphics;
            bitmap: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistory[index2], this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryRound[index2]);
             let mut local2: &Bitmap = &bitmap;
            let mut w: i32 =  num2;
            let mut h: i32 =  num3;
            DrawMod.DrawScaled( local1,  local2, 0, 0, w, h);
            Expression2 = (Graphics) null;
          }
          int[] minicard = this.minicard;
          let mut index3: i32 =  index2;
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.tempbmp[index2]);
          let mut num7: i32 =  this.AddSubPart( tsubpart, x, y, num2, num3, 1);
          minicard[index3] = num7;
          if (this.detailnr == index2)
          {
            DrawMod.DrawRectangle( Expression1, x - 1, y - 1, num2 + 1, num3 + 1, 0, 0,  byte.MaxValue, 185);
            DrawMod.DrawRectangle( Expression1, x - 2, y - 2, num2 + 3, num3 + 3, 0, 0,  byte.MaxValue, 125);
            DrawMod.DrawRectangle( Expression1, x - 3, y - 3, num2 + 5, num3 + 5, 0, 0,  byte.MaxValue, 65);
          }
        }
      }
      if (this.detailnr > -1)
      {
         let mut local3: &Graphics = &Expression1;
        bitmap: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistory[this.detailnr]);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local3,  local4, 660, 160);
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
          DrawMod.DrawTextColoured( Expression1, "Played " + str2, Font::new("Times New Roman", 19f, FontStyle.Bold, GraphicsUnit.Pixel), 715, 625, Color.White);
        }
        else
          DrawMod.DrawTextColoured( Expression1, "Played in round " + Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardHistoryRound[this.detailnr])), Font::new("Times New Roman", 19f, FontStyle.Bold, GraphicsUnit.Pixel), 715, 625, Color.White);
      }
      if (!Information.IsNothing( Expression1))
      {
        Expression1.Dispose();
        Expression1 = (Graphics) null;
      }
      if (Information.IsNothing( Expression2))
        return;
      Expression2.Dispose();
    }

     void docardshand()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
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
      Graphics Expression1 = Graphics.FromImage((Image) this.OwnBitmap);
      Expression1.SmoothingMode = SmoothingMode.AntiAlias;
      Expression1.TextRenderingHint = TextRenderingHint.AntiAlias;
      Expression1.TextContrast = 1;
      DrawMod.DrawBlock( Expression1, 610, 120, 377, 576,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  Expression1, 610, 120, 377, 576, -1, -1);
      this.ClearMouse();
      SimpleList simpleList = SimpleList::new();
      let mut actionCardCounter1: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter;
      let mut num1: i32 =  actionCardCounter1;
      index2: i32;
      for (index2 = 0; index2 <= num1; index2 += 1)
      {
        let mut tweight: i32 =  this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index2]].ColorScheme * 1000 + this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index2];
        simpleList.Add(index2, tweight);
      }
      simpleList.Sort();
      SubPartClass tsubpart;
      Graphics Expression2;
      bitmap1: Bitmap;
      Rectangle trect1;
      if ( this.game.Data.RuleVar[839] == 0.0)
      {
        if (this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter > 64)
        {
          this.OptionsList4Obj = ATListClass::new();
          let mut tlistselect: i32 =  -1;
          let mut num2: i32 =  -1;
          let mut actionCardCounter2: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCardCounter;
          for (index2 = 0; index2 <= actionCardCounter2; index2 += 1)
          {
            num2 += 1;
            if (this.detailnr == index2)
              tlistselect = num2;
            this.OptionsList4Obj.add(this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index2]].Title, index2, Conversions.ToString(this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index2]].PPCost));
          }
          if (this.OptionsList4Id > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, tlistselect);
            this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
          }
          else
          {
            tsubpart =  new ATListSubPartClass(this.OptionsList4Obj, 16, 550, tlistselect, this.game, tHeader: "Action Cards", tShowPair: true, tValueWidth: 150, tbackbitmap: ( this.OwnBitmap), bbx: 10, bby: 150);
            this.OptionsList4Id = this.AddSubPart( tsubpart, 10, 160, 550, 304, 0);
          }
        }
        else
        {
          let mut num3: i32 =   Math.Round(Conversion.Int(  Math.Round(Conversion.Int(Math.Sqrt( (actionCardCounter1 + 1))) + 1.0) * 1.5));
          let mut num4: i32 =   Math.Round(Conversion.Int(520.0 /  num3));
          let mut num5: i32 =   Math.Round(Conversion.Int( num4 * 1.5));
          let mut num6: i32 =  -1;
          let mut num7: i32 =  0;
          let mut counter: i32 =  simpleList.Counter;
          for (let mut index3: i32 =  0; index3 <= counter; index3 += 1)
          {
            index2 = simpleList.Id[index3];
            num6 += 1;
            if (num6 >= num3)
            {
              num6 = 0;
              num7 += 1;
            }
            let mut x: i32 =   Math.Round(13.0 +  num6 * ( num4 * 1.1));
            let mut y: i32 =   Math.Round(156.0 +  num7 * ( num5 * 1.1));
            if (Information.IsNothing( this.tempbmp[index2]))
            {
              this.tempbmp[index2] = new Bitmap(num4, num5);
              this.tempbmp[index2].SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
              Graphics graphics = Graphics.FromImage((Image) this.tempbmp[index2]);
               let mut local1: &Graphics = &graphics;
              bitmap2: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[index2]);
               let mut local2: &Bitmap = &bitmap2;
              let mut w: i32 =  num4;
              let mut h: i32 =  num5;
              DrawMod.DrawScaled( local1,  local2, 0, 0, w, h);
              Expression2 = (Graphics) null;
            }
            int[] minicard = this.minicard;
            let mut index4: i32 =  index2;
            tsubpart =  ButtonPartClass::new(this.tempbmp[index2]);
            let mut num8: i32 =  this.AddSubPart( tsubpart, x, y, num4, num5, 1);
            minicard[index4] = num8;
            if (this.detailnr == index2)
            {
              DrawMod.DrawRectangle( Expression1, x - 1, y - 1, num4 + 1, num5 + 1, 0, 0,  byte.MaxValue, 185);
              DrawMod.DrawRectangle( Expression1, x - 2, y - 2, num4 + 3, num5 + 3, 0, 0,  byte.MaxValue, 125);
              DrawMod.DrawRectangle( Expression1, x - 3, y - 3, num4 + 5, num5 + 5, 0, 0,  byte.MaxValue, 65);
            }
          }
        }
      }
      else
      {
        let mut num9: i32 =   Math.Round(Conversion.Int(1650.0 /  (simpleList.Counter + 1)));
        if (num9 > 110)
          num9 = 110;
        let mut x1: i32 =  25 - num9;
        let mut y1: i32 =  140;
        let mut counter: i32 =  simpleList.Counter;
        for (index2 = 0; index2 <= counter; index2 += 1)
        {
          if (this.detailnr == -1)
            this.detailnr = simpleList.Id[index2];
          x1 += num9;
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
          let mut nr: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[simpleList.Id[index2]];
           let mut local3: &Graphics = &Expression1;
          bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc(nr, size: 2);
           let mut local4: &Bitmap = &bitmap1;
          let mut x2: i32 =  x1;
          let mut y2: i32 =  y1;
          DrawMod.DrawSimple( local3,  local4, x2, y2);
          Rectangle trect2;
          if (this.game.Data.ActionCardObj[nr].MouseOver.Length > 0)
          {
            trect2 = Rectangle::new(x1, y1, 110, 147);
            trect1 = trect2;
            this.AddMouse( trect1, "REGIME CARD", this.game.Data.ActionCardObj[nr].MouseOver + "\r\nClick for close up and play option", simpleList.Id[index2] + 100);
          }
          else
          {
            trect1 = Rectangle::new(x1, y1, 110, 147);
            trect2 = trect1;
            this.AddMouse( trect2, "REGIME CARD", "Click for close up and play option", simpleList.Id[index2] + 100);
          }
        }
      }
      if (this.detailnr > -1)
      {
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
           let mut local5: &Graphics = &Expression1;
          bitmap1 = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]);
           let mut local6: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local5,  local6, 660, 160);
        }
        else
        {
           let mut local7: &Graphics = &Expression1;
          bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]);
           let mut local8: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local7,  local8, 710, 260);
          if (this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]].MouseOver.Length > 0)
          {
            trect1 = Rectangle::new(710, 260, 190, 266);
            let mut trect3: &Rectangle = &trect1
            this.AddMouse( trect3, "SELECTED REGIME CARD", this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]].MouseOver + "\r\nClick on the 'play card' button to play this card.", simpleList.Id[index2] + 100);
          }
          else
          {
            trect1 = Rectangle::new(710, 260, 190, 266);
            let mut trect4: &Rectangle = &trect1
            this.AddMouse( trect4, "SELECTED REGIME CARD", "Click on the 'play card' button to play this card.", simpleList.Id[index2] + 100);
          }
        }
        if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >= this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]].PPCost | this.game.Data.ActionCardObj[this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.detailnr]].PPCost == 0)
        {
          if ( this.game.Data.RuleVar[839] == 0.0)
          {
            tsubpart =  new TextButtonPartClass("PLAY CARD", 200, tBackbitmap: ( this.OwnBitmap), bbx: 715, bby: 625, usefont: usefont, useshadow: flag, tMarcStyle: flag);
            this.B3Id = this.AddSubPart( tsubpart, 715, 625, 200, 35, 1);
          }
          else
          {
            tsubpart =  new TextButtonPartClass("PLAY CARD", 200, "Click to play this card.",  this.OwnBitmap, 700, 560, usefont: usefont, useshadow: flag, tMarcStyle: flag);
            this.B3Id = this.AddSubPart( tsubpart, 700, 560, 200, 35, 1);
          }
        }
        else if ( this.game.Data.RuleVar[839] == 0.0)
        {
          tsubpart =  new TextButtonPartClass("PLAY CARD", 200, tBackbitmap: ( this.OwnBitmap), bbx: 715, bby: 625, tinactive: true, usefont: usefont, useshadow: flag, tMarcStyle: flag);
          this.B3TextId = this.AddSubPart( tsubpart, 715, 625, 200, 35, 1);
        }
        else
        {
          tsubpart =  new TextButtonPartClass("PLAY CARD", 200, "You do not have the PP to play this card.",  this.OwnBitmap, 700, 560, true, usefont: usefont, useshadow: flag, tMarcStyle: flag);
          this.B3TextId = this.AddSubPart( tsubpart, 700, 560, 200, 35, 1);
        }
      }
      if (!Information.IsNothing( Expression1))
      {
        Expression1.Dispose();
        Expression1 = (Graphics) null;
      }
      if (Information.IsNothing( Expression2))
        return;
      Expression2.Dispose();
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
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
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
      this.OptionsList5Obj = ATListClass::new();
      let mut tlistselect: i32 =  -1;
      let mut num1: i32 =  -1;
      let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
      for (let mut tdata: i32 =  0; tdata <= messCounter; tdata += 1)
      {
        if (this.game.Data.RegimeObj[this.game.Data.Turn].MessBackPic[tdata] == -2)
        {
          num1 += 1;
          if (this.detailnr == -1)
            this.detailnr = tdata;
          if (this.detailnr == tdata)
            tlistselect = num1;
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
          this.SubPartList[this.SubpartNr(this.OptionsList5Id)].Refresh(this.OptionsList5Obj, tlistselect);
          this.SubPartFlag[this.SubpartNr(this.OptionsList5Id)] = true;
        }
        else
        {
          let mut tsubpart: SubPartClass =  new ATListSubPartClass(this.OptionsList5Obj, 25, 330, tlistselect, this.game, tHeader: "This rounds reports", tbackbitmap: ( this.OwnBitmap), bbx: 30, bby: 160);
          this.OptionsList5Id = this.AddSubPart( tsubpart, 30, 160, 330, 448, 0);
        }
        if (this.detailnr > -1)
        {
          num3: i32;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.detailnr] > -1)
          {
            let mut index: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.detailnr];
            let mut nr: i32 =  index < 10000 ? this.game.Data.EventPicNr[index] : this.game.Data.HistoricalUnitObj[index - 10000].CommanderSpriteID;
            if (nr > -1)
            {
              let mut w1: i32 =  BitmapStore.GetWidth(nr);
              let mut h1: i32 =  BitmapStore.Getheight(nr);
              if (w1 > 320)
              {
                h1 =  Math.Round( h1 * (320.0 /  w1));
                w1 =  Math.Round( w1 * (320.0 /  w1));
              }
              if (h1 > 200)
              {
                h1 =  Math.Round( h1 * (200.0 /  h1));
                w1 =  Math.Round( w1 * (200.0 /  h1));
              }
              if (this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.detailnr] >= 10000)
              {
                DrawMod.DrawOfficer(graphics, this.game.Data.RegimeObj[this.game.Data.Turn].MessFrontPic[this.detailnr] - 10000,  Math.Round(690.0 -  w1 / 2.0), 140, w1, h1);
              }
              else
              {
                 let mut local1: &Graphics = &graphics;
                bitmap: Bitmap = BitmapStore.GetBitmap(nr);
                 let mut local2: &Bitmap = &bitmap;
                let mut x: i32 =   Math.Round(690.0 -  w1 / 2.0);
                let mut w2: i32 =  w1;
                let mut h2: i32 =  h1;
                DrawMod.DrawScaled( local1,  local2, x, 140, w2, h2);
                DrawMod.DrawRectangle( graphics,  Math.Round(689.0 -  w1 / 2.0), 139, w1 + 2, h1 + 2,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
              }
              num3 = 20 + h1 + 25;
            }
          }
          let mut num4: i32 =   Math.Round(26.0 -  num3 / 20.0);
          if (this.Text2Id == 0)
          {
            let mut num5: i32 =  430;
            let mut num6: i32 =  160 + num3;
            DrawMod.DrawPaperSheet( graphics, num5 - 20, num6 - 10, 560, 20 * num4);
            let mut tsubpart: SubPartClass =  new PaperAreaClass(this.game, 535, num4 - 1,  null, "Description", false, this.game.Data.RegimeObj[this.game.Data.Turn].MessString[this.detailnr], this.game.VicColor8, tItemSize: 20, tbackbitmap: ( this.OwnBitmap), bbx: num5, bby: num6);
            this.Text2Id = this.AddSubPart( tsubpart, num5, num6, 535, 20 * (num4 - 1), 0);
          }
        }
      }
      if (Information.IsNothing( graphics))
        return;
      graphics.Dispose();
    }

     void dostuff()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text4id > 0)
        this.RemoveSubPart(this.Text4id);
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
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      Expression.SmoothingMode = SmoothingMode.AntiAlias;
      Expression.TextRenderingHint = TextRenderingHint.AntiAlias;
      Expression.TextContrast = 1;
      DrawMod.DrawBlock( Expression, 610, 120, 377, 576,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
      DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  Expression, 610, 120, 377, 576, -1, -1);
      this.OptionsListObj = ATListClass::new();
      if (this.detailnr > this.game.Data.ResearchCounter)
        this.detailnr = -1;
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      SubPartClass tsubpart;
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
        let mut tlistselect: i32 =  -1;
        let mut listCount: i32 =  this.OptionsListObj.ListCount;
        for (let mut index: i32 =  0; index <= listCount; index += 1)
        {
          if (this.detailnr == this.OptionsListObj.ListData[index])
            tlistselect = index;
        }
        if (this.OptionsListId > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect);
          this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
        }
        else
        {
          tsubpart =  new ATListSubPartClass(this.OptionsListObj, 30, 170, tlistselect, this.game, tHeader: "Available Research", tbackbitmap: ( this.OwnBitmap), bbx: 410, bby: 160);
          this.OptionsListId = this.AddSubPart( tsubpart, 410, 160, 170, 528, 0);
        }
      }
      this.OptionsList3Obj = ATListClass::new();
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
        let mut tlistselect: i32 =  -1;
        let mut listCount: i32 =  this.OptionsList3Obj.ListCount;
        for (let mut index: i32 =  0; index <= listCount; index += 1)
        {
          if (this.detailnr3 == this.OptionsList3Obj.ListData[index])
            tlistselect = index;
        }
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, tlistselect);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          tsubpart =  new ATListSubPartClass(this.OptionsList3Obj, 30, 170, tlistselect, this.game, tHeader: "Not avail.", tbackbitmap: ( this.OwnBitmap), bbx: 220, bby: 160);
          this.OptionsList3Id = this.AddSubPart( tsubpart, 220, 160, 170, 528, 0);
        }
      }
      num1 = -1;
      let mut num6: i32 =  -1;
      if (this.detailnr2 > this.game.Data.ResearchCounter)
        this.detailnr2 = -1;
      this.OptionsList2Obj = ATListClass::new();
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
      let mut tlistselect1: i32 =  -1;
      let mut listCount1: i32 =  this.OptionsList2Obj.ListCount;
      for (let mut index: i32 =  0; index <= listCount1; index += 1)
      {
        if (this.detailnr2 == this.OptionsList2Obj.ListData[index])
          tlistselect1 = index;
      }
      if (this.OptionsList2Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, tlistselect1);
        this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
      }
      else
      {
        tsubpart =  new ATListSubPartClass(this.OptionsList2Obj, 30, 170, tlistselect1, this.game, tHeader: "Known Research", tHeaderCenter: false, tbackbitmap: ( this.OwnBitmap), bbx: 30, bby: 160);
        this.OptionsList2Id = this.AddSubPart( tsubpart, 30, 160, 170, 528, 0);
      }
      let mut index1: i32 =  this.detailnr;
      if (index1 == -1)
        index1 = this.detailnr2;
      if (index1 == -1)
        index1 = this.detailnr3;
      if (index1 > -1)
      {
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
                DrawMod.DrawText( Expression, "Allies:", this.game.GameFont1, x1, 145);
              num8 = 1;
              x1 += 37;
               let mut local1: &Graphics = &Expression;
              bitmap: Bitmap = BitmapStore.GetBitmap(this.game.Data.RegimeObj[reg2].HQSpriteNr);
               let mut local2: &Bitmap = &bitmap;
              let mut x2: i32 =  x1;
              DrawMod.DrawSimple( local1,  local2, x2, 143);
            }
          }
        }
        if (index1 == this.detailnr)
          DrawMod.DrawSimple( Expression,  this.game.CARD3B, 648, 173);
        else if (index1 == this.detailnr2)
          DrawMod.DrawSimple( Expression,  this.game.CARD2B, 648, 173);
        else if (index1 == this.detailnr3)
          DrawMod.DrawSimple( Expression,  this.game.CARD1B, 648, 173);
        DrawMod.DrawTextVic2( Expression, this.game.Data.ResearchObj[index1].Name, this.game.VicFont1, 675, 201, this.game.VicColor2, this.game.VicColor2Shade);
        if (this.game.Data.ResearchObj[index1].SFTypePic > -1)
        {
          let mut picSpriteId: i32 =  this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].PicSpriteID;
          let mut sidewaysSpriteId: i32 =  this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].SidewaysSpriteID;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 =  this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].ExtraCounter;
            for (let mut index2: i32 =  0; index2 <= extraCounter; index2 += 1)
            {
              if (this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].ExtraCode[index2] == this.game.Data.RegimeObj[this.game.Data.Turn].ExtraGraphicUse)
              {
                picSpriteId = this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].ExtraPicSpriteID[index2];
                sidewaysSpriteId = this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].ExtraSidewaysSpriteID[index2];
              }
            }
          }
          let mut x3: i32 =  665;
          let mut y1: i32 =  240;
          let mut width1: i32 =  260;
          let mut height: i32 =  194;
          index3: i32;
          index4: i32;
          bitmap: Bitmap;
          Rectangle rectangle1;
          Rectangle rectangle2;
          if ( this.game.Data.RuleVar[869] >= 1.0)
          {
            let mut sfTypePic: i32 =  this.game.Data.ResearchObj[index1].SFTypePic;
            index3 =  Math.Round( this.game.Data.RuleVar[873]);
            index4 = 0;
            if ( this.game.Data.RuleVar[848] > 0.0 & this.game.Data.SFTypeObj[sfTypePic].Theater == 2)
            {
              index3 =  Math.Round( this.game.Data.RuleVar[848]);
              index4 = 0;
            }
            if ( this.game.Data.RuleVar[872] > 0.0 & this.game.Data.SFTypeObj[sfTypePic].Theater == 1)
            {
              index3 =  Math.Round( this.game.Data.RuleVar[872]);
              index4 = 0;
            }
            if ( this.game.Data.RuleVar[869] == 3.0)
            {
              let mut nr: i32 =  this.game.Data.LandscapeTypeObj[index3].BasicPicID[index4];
               let mut local3: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(nr);
               let mut local4: &Bitmap = &bitmap;
              rectangle1 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
              let mut srcrect: &Rectangle = &rectangle1
              rectangle2 = Rectangle::new(x3, y1, width1, height);
              let mut destrect: &Rectangle = &rectangle2
              DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
            }
            else
            {
              if ( this.game.Data.RuleVar[869] == 1.0)
              {
                let mut nr: i32 =  this.game.Data.LandscapeTypeObj[index3].SidewaysSPriteID1[index4];
                 let mut local5: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr);
                 let mut local6: &Bitmap = &bitmap;
                rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(x3, y1, width1, height);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
              }
              let mut nr1: i32 =  this.game.Data.LandscapeTypeObj[index3].SidewaysSPriteID2[index4];
               let mut local7: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(nr1);
               let mut local8: &Bitmap = &bitmap;
              rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr1));
              let mut srcrect1: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(x3, y1, width1, height);
              let mut destrect1: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2( local7,  local8, srcrect1, destrect1);
            }
          }
          let mut turn: i32 =  this.game.Data.Turn;
          let mut red: i32 =  this.game.Data.RegimeObj[turn].Red;
          let mut green: i32 =  this.game.Data.RegimeObj[turn].Green;
          let mut blue: i32 =  this.game.Data.RegimeObj[turn].Blue;
          switch (this.game.Data.SFTypeObj[this.game.Data.ResearchObj[index1].SFTypePic].BaseColor)
          {
            case 0:
               let mut local9: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local10: &Bitmap = &bitmap;
              let mut x4: i32 =  x3;
              let mut y2: i32 =  y1;
              let mut w1: i32 =  width1;
              let mut h1: i32 =  height;
              DrawMod.DrawScaled( local9,  local10, x4, y2, w1, h1);
              break;
            case 1:
               let mut local11: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local12: &Bitmap = &bitmap;
              let mut x5: i32 =  x3;
              let mut y3: i32 =  y1;
              let mut w2: i32 =  width1;
              let mut h2: i32 =  height;
              let mut width2: i32 =  BitmapStore.GetWidth(picSpriteId);
              let mut origh1: i32 =  BitmapStore.Getheight(picSpriteId);
              double r1 =  ( red / 256f);
              double g1 =  ( green / 256f);
              double b1 =  ( blue / 256f);
              DrawMod.DrawScaledColorized2( local11,  local12, x5, y3, w2, h2, width2, origh1,  r1,  g1,  b1, 1f);
              break;
            case 2:
              let mut red2: i32 =  this.game.Data.RegimeObj[turn].Red2;
              let mut green2: i32 =  this.game.Data.RegimeObj[turn].Green2;
              let mut blue2: i32 =  this.game.Data.RegimeObj[turn].Blue2;
               let mut local13: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local14: &Bitmap = &bitmap;
              let mut x6: i32 =  x3;
              let mut y4: i32 =  y1;
              let mut w3: i32 =  width1;
              let mut h3: i32 =  height;
              let mut width3: i32 =  BitmapStore.GetWidth(picSpriteId);
              let mut origh2: i32 =  BitmapStore.Getheight(picSpriteId);
              double r2 =  ( red2 / 256f);
              double g2 =  ( green2 / 256f);
              double b2 =  ( blue2 / 256f);
              DrawMod.DrawScaledColorized2( local13,  local14, x6, y4, w3, h3, width3, origh2,  r2,  g2,  b2, 1f);
              break;
            case 3:
              let mut red3: i32 =  this.game.Data.RegimeObj[turn].Red3;
              let mut green3: i32 =  this.game.Data.RegimeObj[turn].Green3;
              let mut blue3: i32 =  this.game.Data.RegimeObj[turn].Blue3;
               let mut local15: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local16: &Bitmap = &bitmap;
              let mut x7: i32 =  x3;
              let mut y5: i32 =  y1;
              let mut w4: i32 =  width1;
              let mut h4: i32 =  height;
              let mut width4: i32 =  BitmapStore.GetWidth(picSpriteId);
              let mut origh3: i32 =  BitmapStore.Getheight(picSpriteId);
              double r3 =  ( red3 / 256f);
              double g3 =  ( green3 / 256f);
              double b3 =  ( blue3 / 256f);
              DrawMod.DrawScaledColorized2( local15,  local16, x7, y5, w4, h4, width4, origh3,  r3,  g3,  b3, 1f);
              break;
            case 4:
              let mut red4: i32 =  this.game.Data.RegimeObj[turn].Red4;
              let mut green4: i32 =  this.game.Data.RegimeObj[turn].Green4;
              let mut blue4: i32 =  this.game.Data.RegimeObj[turn].Blue4;
               let mut local17: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local18: &Bitmap = &bitmap;
              let mut x8: i32 =  x3;
              let mut y6: i32 =  y1;
              let mut w5: i32 =  width1;
              let mut h5: i32 =  height;
              let mut width5: i32 =  BitmapStore.GetWidth(picSpriteId);
              let mut origh4: i32 =  BitmapStore.Getheight(picSpriteId);
              double r4 =  ( red4 / 256f);
              double g4 =  ( green4 / 256f);
              double b4 =  ( blue4 / 256f);
              DrawMod.DrawScaledColorized2( local17,  local18, x8, y6, w5, h5, width5, origh4,  r4,  g4,  b4, 1f);
              break;
            case 5:
               let mut local19: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local20: &Bitmap = &bitmap;
              let mut x9: i32 =  x3;
              let mut y7: i32 =  y1;
              let mut w6: i32 =  width1;
              let mut h6: i32 =  height;
              let mut width6: i32 =  BitmapStore.GetWidth(picSpriteId);
              let mut origh5: i32 =  BitmapStore.Getheight(picSpriteId);
              double r5 =  ( (red + 392) / 1024f);
              double g5 =  ( (green + 392) / 1024f);
              double b5 =  ( (blue + 392) / 1024f);
              DrawMod.DrawScaledColorized2( local19,  local20, x9, y7, w6, h6, width6, origh5,  r5,  g5,  b5, 1f);
              break;
            case 6:
               let mut local21: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(picSpriteId);
               let mut local22: &Bitmap = &bitmap;
              let mut x10: i32 =  x3;
              let mut y8: i32 =  y1;
              let mut w7: i32 =  width1;
              let mut h7: i32 =  height;
              let mut width7: i32 =  BitmapStore.GetWidth(picSpriteId);
              let mut origh6: i32 =  BitmapStore.Getheight(picSpriteId);
              double r6 =  ( (red + 80) / 512f);
              double g6 =  ( (green + 200) / 512f);
              double b6 =  ( (blue + 80) / 512f);
              DrawMod.DrawScaledColorized2( local21,  local22, x10, y8, w7, h7, width7, origh6,  r6,  g6,  b6, 1f);
              break;
          }
          if ( this.game.Data.RuleVar[870] > 0.0 & !Information.IsNothing( BitmapStore.GetBitmap(sidewaysSpriteId)))
          {
             let mut local23: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
             let mut local24: &Bitmap = &bitmap;
            let mut x11: i32 =  x3;
            let mut y9: i32 =  y1;
            let mut w8: i32 =  width1;
            let mut h8: i32 =  height;
            DrawMod.DrawScaled( local23,  local24, x11, y9, w8, h8);
          }
          if ( this.game.Data.RuleVar[869] >= 1.0 &  this.game.Data.RuleVar[869] < 3.0)
          {
            let mut nr: i32 =  this.game.Data.LandscapeTypeObj[index3].SidewaysSPriteID3[index4];
             let mut local25: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(nr);
             let mut local26: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x3, y1, width1, height);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local25,  local26, srcrect, destrect);
          }
           let mut local27: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(this.game.ACTIONFRAME);
           let mut local28: &Bitmap = &bitmap;
          DrawMod.DrawSimple( local27,  local28, 665, 240);
        }
        if (this.game.Data.ResearchObj[index1].PreReq > -1)
        {
          tstring: String = "PreReq: " + this.game.Data.ResearchObj[this.game.Data.ResearchObj[index1].PreReq].Name;
          DrawMod.DrawTextVic2( Expression, tstring, this.game.VicFont2, 675, 450, Color.Black, Color.Transparent);
        }
        if (this.game.Data.ResearchObj[index1].PreReq2 > -1)
        {
          tstring: String = "PreReq: " + this.game.Data.ResearchObj[this.game.Data.ResearchObj[index1].PreReq2].Name;
          DrawMod.DrawTextVic2( Expression, tstring, this.game.VicFont2, 675, 465, Color.Black, Color.Transparent);
        }
        tstring1: String = this.game.Data.ResearchObj[index1].CostType <= -1 ? Conversion.Str( Conversion.Int( this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] * this.game.Data.ResCostMod)) + "pp" : Conversion.Str( Conversion.Int( this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup] * this.game.Data.ResCostMod)) + " " + this.game.Data.RegimeSlotName[this.game.Data.ResearchObj[index1].CostType];
        DrawMod.DrawTextVic2( Expression, tstring1, this.game.VicFont1, 670, 480, Color.Black, Color.Transparent);
        if (this.game.Data.ResearchObj[index1].CostType == -1)
        {
          if (this.detailnr2 == -1 & this.detailnr3 == -1 &  Conversion.Int(this.game.Data.ResCostMod *  this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup]) <=  this.game.Data.RegimeObj[this.regnr].ResPts)
          {
            tsubpart =  new TextButtonPartClass("Buy", 100, "Buy selected researchfield",  this.OwnBitmap, 750, 635);
            this.B2Id = this.AddSubPart( tsubpart, 750, 635, 100, 35, 1);
          }
        }
        else if (this.detailnr2 == -1 & this.detailnr3 == -1 &  Conversion.Int(this.game.Data.ResCostMod *  this.game.Data.ResearchObj[index1].PointCost[this.game.Data.PeopleObj[this.pplnr].PeopleGroup]) <=  this.game.Data.RegimeObj[this.regnr].RegimeSlot[this.game.Data.ResearchObj[index1].CostType])
        {
          tsubpart =  new TextButtonPartClass("Buy", 100, "Buy selected researchfield",  this.OwnBitmap, 750, 635);
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
            tsubpart =  new TextButtonPartClass("Give to Ally", 200, "Give selected researchfield",  this.OwnBitmap, 700, 635);
            this.BAllyId = this.AddSubPart( tsubpart, 700, 635, 200, 35, 1);
          }
        }
        tsubpart =  new TextAreaClass(this.game, 280, 4, this.game.VicFont3, "", false, this.game.Data.ResearchObj[index1].Text, Color.Black, tbackbitmap: ( this.OwnBitmap), bbx: 665, bby: 510, tHideShade: true, tBlockScroller: true);
        this.Text4id = this.AddSubPart( tsubpart, 665, 510, 280, 80, 0);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
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
          windowReturnClass.AddCommand(6, 0);
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
      let mut upperBound: i32 =  this.tempbmp.GetUpperBound(0);
      for (let mut index: i32 =  0; index <= upperBound; index += 1)
      {
        if (!Information.IsNothing( this.tempbmp[index]))
        {
          this.tempbmp[index].Dispose();
          this.tempbmp[index] = (Bitmap) null;
        }
      }
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
              windowReturnClass1.AddCommand(6, 0);
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
